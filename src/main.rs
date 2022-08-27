use tinyrenderer::tga::{Image, RGBA};

// const WHITE: RGBA = RGBA { r: 255, g: 255, b: 255, a: 255 };
const RED: RGBA = RGBA { r: 255, g: 0, b: 0, a: 255 };

fn main() {
    let mut img = Image::new(100, 100);

    img.set(52, 41, RED).unwrap();
    img.write_to_file("main.tga", true, false).unwrap();
}
