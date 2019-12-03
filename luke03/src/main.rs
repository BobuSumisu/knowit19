use image;
use std::fs;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let num_chars = input.len();

    if fs::metadata("output").is_ok() {
        fs::remove_dir_all("output/").unwrap();
    }
    fs::create_dir("output").unwrap();

    for width in 1..num_chars {
        if num_chars % width == 0 {
            let height = num_chars / width;

            let buffer = input
                .as_bytes()
                .chunks(width)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|b| if *b == 0x31 { 255 } else { 0 })
                        .collect::<Vec<u8>>()
                })
                .flatten()
                .collect::<Vec<u8>>();

            let path = format!("output/{}x{}.png", width, height);

            image::save_buffer(&path, &buffer, width as u32, height as u32, image::Gray(8))
                .unwrap();

            println!("image saved: {}", path);
        }
    }
}
