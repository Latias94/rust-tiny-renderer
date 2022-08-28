use tinyrenderer::math::Vec2;
use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::{BLACK, GREEN, RED, WHITE};

fn main() {
    let mut rasterizer = Rasterizer::new(200, 200);
    rasterizer.clear(BLACK);
    let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
    let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
    let t2 = [
        Vec2::new(180, 150),
        Vec2::new(120, 160),
        Vec2::new(130, 180),
    ];

    rasterizer.triangle(t0[0],t0[1],t0[2], RED);
    rasterizer.triangle(t1[0],t1[1],t1[2], WHITE);
    rasterizer.triangle(t2[0],t2[1],t2[2], GREEN);
    rasterizer.write_to_file("main.tga");
}
