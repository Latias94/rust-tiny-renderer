use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::{BLACK, RED, WHITE};

fn main() {
    let mut rasterizer = Rasterizer ::new(100, 100);
    rasterizer.clear(BLACK);
    rasterizer.line(13, 20, 80, 40, WHITE);
    rasterizer.line(20, 13, 40, 80, RED);
    rasterizer.line(80, 40, 13, 20, RED);
    rasterizer.write_to_file("main.tga");
}
