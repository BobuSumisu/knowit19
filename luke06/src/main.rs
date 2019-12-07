use image::Rgb;

fn main() {
    let mut img = image::open("input/input.png").unwrap();
    let img = img.as_mut_rgb8().unwrap();

    let mut prev = None;

    for pixel in img.pixels_mut() {
        let tmp = *pixel;

        if let Some(prev) = prev {
            *pixel = xor(*pixel, prev);
        }

        prev = Some(tmp);
    }

    img.save("output/output.png").unwrap();
}

fn xor(a: Rgb<u8>, b: Rgb<u8>) -> Rgb<u8> {
    Rgb([a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2]])
}
