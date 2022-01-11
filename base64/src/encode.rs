pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    // return std::str::from_utf8(input.as_ref()).unwrap().to_string();

    const BASE64_ALPHABET: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
        b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
        b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
        b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3',
        b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
    ];

    let charactoers = input.as_ref();

    let mut base64_output = Vec::new();
    let mut counter = 0;
    while counter + 3 <= charactoers.len() {
        let first_base64_character = extract_first_character_bits(charactoers[counter]);
        let second_base64_character = extract_second_character_bits(charactoers[counter], charactoers[counter + 1]);
        let third_base64_character = extract_third_character_bits(charactoers[counter + 1], charactoers[counter + 2]);
        let fourth_base64_character = extract_fourth_character_bits(charactoers[counter + 2]);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            BASE64_ALPHABET[third_base64_character as usize],
            BASE64_ALPHABET[fourth_base64_character as usize],
        ]);

        counter += 3;
    }

    if counter + 1 == charactoers.len() {
        let first_base64_character = extract_first_character_bits(charactoers[counter]);
        let second_base64_character = extract_second_character_bits(charactoers[counter], 0);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            b'=',
            b'=',
        ]);
    } else if counter + 2 == charactoers.len() {
        let first_base64_character = extract_first_character_bits(charactoers[counter]);
        let second_base64_character = extract_second_character_bits(charactoers[counter], charactoers[counter + 1]);
        let third_base64_character = extract_third_character_bits(charactoers[counter + 1], 0);

        base64_output.append(&mut vec![
            BASE64_ALPHABET[first_base64_character as usize],
            BASE64_ALPHABET[second_base64_character as usize],
            BASE64_ALPHABET[third_base64_character as usize],
            b'=',
        ]);
    }

    std::str::from_utf8(&base64_output).unwrap().to_string()
}

#[test]
fn test_encode() {
    assert_eq!(encode("abcde"), "YWJjZGU=");
    assert_eq!(encode("あいう"), "44GC44GE44GG");
    assert_eq!(encode("😉🥳🤔"), "8J+YifCfpbPwn6SU");
}

fn extract_first_character_bits(first_byte: u8) -> u8 {
    (first_byte & 0b11111100) >> 2
}

#[test]
fn test_extract_first_character_bits() {
    assert_eq!(
        extract_first_character_bits(b'a'),
        0b00011000
    );
}

fn extract_second_character_bits(first_byte: u8, second_byte: u8) -> u8 {
    ((first_byte & 0b00000011) << 4) | ((second_byte & 0b11110000) >> 4)
}

#[test]
fn test_extract_second_character_bits() {
    assert_eq!(
        extract_second_character_bits(b'a', b'b'),
        0b00010110
    );
}

fn extract_third_character_bits(second_byte: u8, third_byte: u8) -> u8 {
    ((second_byte & 0b00001111) << 2) | ((third_byte & 0b11000000) >> 6)
}

#[test]
fn test_extract_third_character_bits() {
    assert_eq!(
        extract_third_character_bits(b'b', b'c'),
        0b00001001
    );
}

fn extract_fourth_character_bits(third_byte: u8) -> u8 {
    third_byte & 0b00111111
}

#[test]
fn test_extract_fourth_character_bits() {
    assert_eq!(
        extract_fourth_character_bits(b'd'),
        0b00100100
    );
}
