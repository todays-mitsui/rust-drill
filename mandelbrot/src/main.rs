extern crate num;
extern crate image;

use std::io::Write;
use std::str::FromStr;
use std::fs::File;
use num::Complex;
use image::png::PNGEncoder;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
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

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.00625"),
        Some(Complex { re: 1.25, im: -0.00625 })
    );

    assert_eq!(
        parse_complex(",-0.00625"),
        None
    );
}

#[derive(Eq, PartialEq, Debug)]
struct Bounds {
    width: usize,
    height: usize,
}

impl From<(u64, u64)> for Bounds {
    fn from(pair: (u64, u64)) -> Bounds {
        Bounds { width: pair.0 as usize, height: pair.1 as usize }
    }
}

#[test]
fn test_bounds_from_tuple() {
    assert_eq!(
        Bounds::from((42, 100)),
        Bounds { width: 42, height: 100 }
    );
}

#[derive(Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn pixel_to_point(
    bounds: &Bounds,
    pixcel: &Point,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + (pixcel.x as f64) * width / (bounds.width as f64),
        im: upper_left.im - (pixcel.y as f64) * height / (bounds.height as f64),
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            &Bounds { width: 100, height: 100 },
            &Point { x: 25, y: 75 },
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

fn render(
    pixels: &mut [u8],
    bounds: &Bounds,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.width * bounds.height);

    for row in 0 .. bounds.height {
        for column in 0 .. bounds.width {
            let point = pixel_to_point(&bounds, &Point { x: column, y: row }, upper_left, lower_right);

            pixels[row * bounds.width + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - (count as u8),
            };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: Bounds) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.width as u32,
        bounds.height as u32,
        image::ColorType::Gray(8),
    )?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT"
        ).unwrap();

        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        ).unwrap();

        std::process::exit(1);
    }

    let pair = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let bounds = Bounds::from(pair);

    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");

    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.width * bounds.height];

    // render(&mut pixels, bounds, upper_left, lower_right);

    let threads = 8;
    let rows_per_band = bounds.height / threads + 1;

    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.width).collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.width;
                let band_bounds = Bounds { width: bounds.width, height };
                let band_upper_left = pixel_to_point(&bounds, &Point { x: 0, y: top }, upper_left, lower_right);
                let band_lower_right = pixel_to_point(&bounds, &Point { x: bounds.width, y: top + height }, upper_left, lower_right);

                spawner.spawn(move || {
                    render(band, &band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");
}
