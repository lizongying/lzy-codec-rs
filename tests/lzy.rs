use lzy_codec::lzy;

// 1. valid_unicode 方法测试
#[test]
fn test_valid_unicode_valid_code_points() {
    // 合法 Unicode：ASCII、中文、Emoji
    assert!(lzy::valid_unicode(b'A' as u32)); // 0x41
    assert!(lzy::valid_unicode('中' as u32)); // 0x4E2D
    assert!(lzy::valid_unicode('😀' as u32)); // 0x1F600
    assert!(lzy::valid_unicode(0));
    assert!(lzy::valid_unicode(lzy::UNICODE_MAX));
}

#[test]
fn test_valid_unicode_invalid_surrogates() {
    // 代理区字符（无效）
    assert!(!lzy::valid_unicode(lzy::SURROGATE_MIN));
    assert!(!lzy::valid_unicode(lzy::SURROGATE_MAX));
    assert!(!lzy::valid_unicode(0xDBFF));
}

#[test]
fn test_valid_unicode_out_of_range() {
    // 超出 Unicode 范围（无效）
    assert!(!lzy::valid_unicode(u32::MAX));
    assert!(!lzy::valid_unicode(lzy::UNICODE_MAX + 1));
}

// 2. 编码方法测试
#[test]
fn test_encode_ascii_chars() {
    let chars = vec!['A' as u32, 'B' as u32, 'C' as u32];
    let encoded = lzy::encode(&chars);
    // 预期 ASCII 字节
    let expected = vec![b'A', b'B', b'C'];
    assert_eq!(encoded, expected);
}

#[test]
fn test_encode_from_str_chinese() {
    let s = "中国";
    let encoded = lzy::encode_from_string(s);
    // 闭环测试：编码后解码与原字符串一致
    let decoded = lzy::decode_to_string(&encoded).unwrap();
    assert_eq!(decoded, s);
}

#[test]
fn test_encode_from_str_emoji() {
    let s = "😀测试Rust";
    let encoded = lzy::encode_from_string(s);
    let decoded = lzy::decode_to_string(&encoded).unwrap();
    assert_eq!(decoded, s);
}

// 3. 解码方法测试
#[test]
fn test_decode_valid_bytes() {
    let s = "ABC中文123";
    let encoded = lzy::encode_from_string(s);
    let decoded_chars = lzy::decode(&encoded).unwrap();
    let decoded_str: String = decoded_chars
        .into_iter()
        .map(|cp| std::char::from_u32(cp).ok_or("无效的 Unicode 码点")  .unwrap()  )
        .collect();
    assert_eq!(decoded_str, s);
}

#[test]
fn test_decode_to_string_valid_bytes() {
    let test_cases = vec!["ASCII", "中文测试", "😀Emoji", "混合测试！123"];
    for &case in &test_cases {
        let encoded = lzy::encode_from_string(case);
        let decoded = lzy::decode_to_string(&encoded).unwrap();
        assert_eq!(decoded, case);
    }
}

// 4. 异常场景测试
#[test]
fn test_decode_empty_bytes() {
    let empty_bytes = Vec::new();
    let result = lzy::decode(&empty_bytes);
    let error_msg = lzy::ERROR_UNICODE.to_string();
    assert!(matches!(result, Err(e) if e == error_msg));
}

#[test]
fn test_decode_invalid_byte_sequence() {
    // 无效字节：无起始单字节（全部高位为1）
    let invalid_bytes = vec![0x80, 0x81, 0x82];
    let result = lzy::decode(&invalid_bytes);
    let error_msg = lzy::ERROR_UNICODE.to_string();
    assert!(matches!(result, Err(e) if e == error_msg));
}

#[test]
fn test_decode_invalid_unicode() {
    // 编码无效代理区字符，再解码验证异常
    let encoded = lzy::encode(&[lzy::SURROGATE_MIN]);
    let result = lzy::decode(&encoded);
    let error_msg = lzy::ERROR_UNICODE.to_string();
    assert!(matches!(result, Err(e) if e == error_msg));
}
