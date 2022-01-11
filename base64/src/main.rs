mod encode;
mod decode;

fn main() {
    let original = "abcde";
    let encoded = encode::encode(original);
    println!("\"{}\" == Base64 encode ==> \"{}\"", original, encoded);

    let base64 = "YWJjZGU=";
    match decode::decode(base64) {
        Some(decoded) => println!("\"{}\" <== Base64 decode == \"{}\"", decoded, base64),
        None => println!("\"{}\" is Invalid Base64 Str.", base64),
    }
}

#[test]
fn test_main() {
    assert_eq!(encode::encode("abcde"), "YWJjZGU=");
    assert_eq!(encode::encode("あいう"), "44GC44GE44GG");
    assert_eq!(encode::encode("😉🥳🤔"), "8J+YifCfpbPwn6SU");
}
