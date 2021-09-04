extern crate num;

use std::str::FromStr;
use num::Complex;

fn main() {
    let c = Complex { re: 0.5, im: 0.5 };
    println!("{:?}", complex_square_add_loop(c, 128));
}

fn complex_square_add_loop(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>(""        , ','), None);
    assert_eq!(parse_pair::<i32>("10,"     , ','), None);
    assert_eq!(parse_pair::<i32>(",20"     , ','), None);
    assert_eq!(parse_pair::<i32>("10,20abc", ','), None);
    assert_eq!(parse_pair::<i32>("10,20"   , ','), Some((10, 20)));
}
