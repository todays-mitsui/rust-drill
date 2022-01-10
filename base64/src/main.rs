mod encode;
mod decode;

fn main() {
    println!("main");
}

#[test]
fn test_main() {
    assert_eq!(encode::encode("abcde"), "YWJjZGU=");
    assert_eq!(encode::encode("あいう"), "44GC44GE44GG");
    assert_eq!(encode::encode("😉🥳🤔"), "8J+YifCfpbPwn6SU");
}
