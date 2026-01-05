use lzy_codec::lzy;

fn main() {
    let message = "hello，世界😊".as_bytes();
    println!("Original: {:?}", message);

    let encoded = lzy::encode_from_bytes(message);
    println!("Encoded: {:?}", encoded);

    match encoded {
        Ok(encoded_bytes) => match lzy::decode_to_bytes(&encoded_bytes) {
            Ok(decoded_bytes) => match String::from_utf8(decoded_bytes) {
                Ok(decoded_str) => {
                    println!("Decoded: {:?}", decoded_str);
                }
                Err(utf8_err) => {
                    eprintln!("Failed to convert decoded bytes to string: {}", utf8_err);
                }
            },
            Err(decode_err) => {
                eprintln!("Failed to decode encoded bytes: {}", decode_err);
            }
        },
        Err(encode_err) => {
            eprintln!("Failed to encode bytes: {}", encode_err);
        }
    }
}
