fn decode<T: AsRef<[u8]>>(input: T) -> Option<String> {
    if input.as_ref().len() % 4 != 0 { return None; }

    return Some("aaa".to_string());
}

#[test]
fn test_decode() {
    assert_eq!(decode("ab"), None);
    assert_eq!(decode("YWJjZGU="), Some("abcde".to_string()));
    assert_eq!(decode("44GC44GE44GG"), Some("あいう".to_string()));
    assert_eq!(decode("8J+YifCfpbPwn6SU"), Some("😉🥳🤔".to_string()));
}
