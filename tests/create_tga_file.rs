use tinyrenderer::tga::{Image, RGBA};

#[test]
fn test_tga() {
    let mut img = Image::new(640, 480);
    for y in 0u32..480 {
        for x in 0u32..640 {
            let r = ((x ^ y) % 256) as u8;
            let g = ((x + y) % 256) as u8;
            let b = ((y.wrapping_sub(x)) % 256) as u8;
            let a = 255_u8;
            img.set(x as i32, y as i32, RGBA { r, g, b, a }).unwrap();
        }
    }
    img.write_to_file("test.tga", true, false).unwrap();
}
