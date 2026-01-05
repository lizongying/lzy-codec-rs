pub const SURROGATE_MIN: u32 = 0xD800;
pub const SURROGATE_MAX: u32 = 0xDFFF;
pub const UNICODE_MAX: u32 = 0x10FFFF;
pub const ERROR_UNICODE: &str = "invalid unicode";

pub fn valid_unicode(r: u32) -> bool {
    (0..SURROGATE_MIN).contains(&r) || (SURROGATE_MAX < r && r <= UNICODE_MAX)
}

pub fn encode(input_runes: &[u32]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input_runes.len());
    for &r in input_runes {
        if r < 0x80 {
            output.push((r & 0xFF) as u8);
        } else if r < 0x4000 {
            output.push(((r >> 7) & 0xFF) as u8);
            output.push(((0x80 | (r & 0x7F)) & 0xFF) as u8);
        } else {
            output.push(((r >> 14) & 0xFF) as u8);
            output.push(((0x80 | ((r >> 7) & 0x7F)) & 0xFF) as u8);
            output.push(((0x80 | (r & 0x7F)) & 0xFF) as u8);
        }
    }
    output
}

pub fn encode_from_string(input_str: &str) -> Vec<u8> {
    let runes: Vec<u32> = input_str.chars().map(|c| c as u32).collect();
    encode(&runes)
}

pub fn encode_from_bytes(input_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let input_str = std::str::from_utf8(input_bytes).map_err(|e| e.to_string())?;
    Ok(encode_from_string(input_str))
}

pub fn decode(input_bytes: &[u8]) -> Result<Vec<u32>, String> {
    if input_bytes.is_empty() {
        return Err(ERROR_UNICODE.to_string());
    }
    let mut start_idx = None;
    for (i, &b) in input_bytes.iter().enumerate() {
        if (b & 0x80) == 0 {
            start_idx = Some(i);
            break;
        }
    }
    let start_idx = start_idx.ok_or(ERROR_UNICODE)?;
    let mut output = Vec::new();
    let mut r: u32 = 0;
    for (i, &b) in input_bytes.iter().enumerate().skip(start_idx) {
        let b = b as u32;
        if (b >> 7) == 0 {
            if i > start_idx {
                if !valid_unicode(r) {
                    return Err(ERROR_UNICODE.to_string());
                }
                output.push(r);
            }
            r = b;
        } else {
            if r > (UNICODE_MAX >> 7) {
                return Err(ERROR_UNICODE.to_string());
            }
            r = (r << 7) | (b & 0x7F);
        }
    }
    if !valid_unicode(r) {
        return Err(ERROR_UNICODE.to_string());
    }
    output.push(r);
    Ok(output)
}

pub fn decode_to_string(input_bytes: &[u8]) -> Result<String, String> {
    let runes = decode(input_bytes)?;
    let chars: Result<Vec<char>, String> = runes
        .iter()
        .map(|&r| char::try_from(r).map_err(|e| e.to_string()))
        .collect();
    Ok(chars?.into_iter().collect())
}

pub fn decode_to_bytes(input_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let s = decode_to_string(input_bytes)?;
    Ok(s.into_bytes())
}
