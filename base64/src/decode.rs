use std::collections::HashMap;

pub fn decode<T: AsRef<[u8]>>(input: T) -> Option<String> {
    let charactoers = input.as_ref();

    if charactoers.len() % 4 != 0 { return None; }

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
    let base64_index: HashMap<&u8, u8> = BASE64_ALPHABET.iter()
        .enumerate()
        .map(|(i, c)| (c, i as u8))
        .collect();

    let mut output = Vec::new();
    let mut counter = 0;
    while counter + 4 < charactoers.len() {
        let u8s = match (base64_index.get(&charactoers[counter]), base64_index.get(&charactoers[counter+1]), base64_index.get(&charactoers[counter+2]), base64_index.get(&charactoers[counter+3])) {
            (Some(first_bits), Some(second_bits), Some(third_bits), Some(fourth_bits)) => recombine(*first_bits, *second_bits, *third_bits, *fourth_bits),
            _ => return None,
        };

        output.append(&mut vec![u8s[0], u8s[1], u8s[2]]);

        counter += 4;
    }

    if counter != charactoers.len() && charactoers[counter+2] == b'=' {
        let u8s = match (base64_index.get(&charactoers[counter]), base64_index.get(&charactoers[counter+1])) {
            (Some(first_bits), Some(second_bits)) => recombine(*first_bits, *second_bits, 0, 0),
            _ => return None,
        };

        output.append(&mut vec![u8s[0]]);
    } else if counter != charactoers.len() && charactoers[counter+3] == b'=' {
        let u8s = match (base64_index.get(&charactoers[counter]), base64_index.get(&charactoers[counter+1]), base64_index.get(&charactoers[counter+2])) {
            (Some(first_bits), Some(second_bits), Some(third_bits)) => recombine(*first_bits, *second_bits, *third_bits, 0),
            _ => return None,
        };

        output.append(&mut vec![u8s[0], u8s[1]]);
    } else {
        let u8s = match (base64_index.get(&charactoers[counter]), base64_index.get(&charactoers[counter+1]), base64_index.get(&charactoers[counter+2]), base64_index.get(&charactoers[counter+3])) {
            (Some(first_bits), Some(second_bits), Some(third_bits), Some(fourth_bits)) => recombine(*first_bits, *second_bits, *third_bits, *fourth_bits),
            _ => return None,
        };

        output.append(&mut vec![u8s[0], u8s[1], u8s[2]]);
    }

    Some(std::str::from_utf8(&output).unwrap().to_string())
}

#[test]
fn test_decode() {
    assert_eq!(decode("ab"), None);
    assert_eq!(decode("YWJjZGU="), Some("abcde".to_string()));
    assert_eq!(decode("44GC44GE44GG"), Some("あいう".to_string()));
    assert_eq!(decode("8J+YifCfpbPwn6SU"), Some("😉🥳🤔".to_string()));
}

fn recombine(first_bits: u8, second_bits: u8, third_bits: u8, fourth_bits: u8) -> [u8; 3] {
    let first_character = first_bits << 2 | (second_bits & 0b00110000) >> 4;
    let second_character = (second_bits & 0b00001111) << 4 | (third_bits & 0b00111100) >> 2;
    let third_character = (third_bits & 0b00000011) << 6 | fourth_bits;

    return [first_character, second_character, third_character];
}
