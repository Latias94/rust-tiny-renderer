use tinyrenderer::math::Vec2;
use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::{BLACK, GREEN, RED, WHITE};

#[test]
fn test_line() {
    let mut rasterizer = Rasterizer::new(100, 100);
    rasterizer.clear(BLACK.into());
    rasterizer.line(Vec2::new(13, 20), Vec2::new(80, 40), WHITE.into());
    rasterizer.line(Vec2::new(20, 13), Vec2::new(40, 80), RED.into());
    rasterizer.line(Vec2::new(80, 40), Vec2::new(13, 20), RED.into());
    rasterizer.write_to_file("test.png");
}

#[test]
fn test_triangle() {
    let mut rasterizer = Rasterizer::new(200, 200);
    rasterizer.clear(BLACK.into());
    let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
    let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
    let t2 = [
        Vec2::new(180, 150),
        Vec2::new(120, 160),
        Vec2::new(130, 180),
    ];
    rasterizer.triangle(t0[0], t0[1], t0[2], RED.into());
    rasterizer.triangle(t1[0], t1[1], t1[2], WHITE.into());
    rasterizer.triangle(t2[0], t2[1], t2[2], GREEN.into());
    rasterizer.write_to_file("test.png");
}

#[test]
fn test_triangle_1() {
    let mut rasterizer = Rasterizer::new(200, 200);
    rasterizer.clear(BLACK.into());
    let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
    let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
    let t2 = [
        Vec2::new(180, 150),
        Vec2::new(120, 160),
        Vec2::new(130, 180),
    ];

    rasterizer.triangle_test_1(t0[0], t0[1], t0[2]);
    rasterizer.triangle_test_1(t1[0], t1[1], t1[2]);
    rasterizer.triangle_test_1(t2[0], t2[1], t2[2]);
    rasterizer.write_to_file("test.png");
}

#[test]
// draw bottom part of triangle
fn test_triangle_2() {
    let mut rasterizer = Rasterizer::new(200, 200);
    rasterizer.clear(BLACK.into());
    let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
    let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
    let t2 = [
        Vec2::new(180, 150),
        Vec2::new(120, 160),
        Vec2::new(130, 180),
    ];

    rasterizer.triangle_test_2(t0[0], t0[1], t0[2]);
    rasterizer.triangle_test_2(t1[0], t1[1], t1[2]);
    rasterizer.triangle_test_2(t2[0], t2[1], t2[2]);
    rasterizer.write_to_file("test.png");
}

#[test]
// draw fill triangle
fn test_triangle_3() {
    let mut rasterizer = Rasterizer::new(200, 200);
    rasterizer.clear(BLACK.into());
    let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
    let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
    let t2 = [
        Vec2::new(180, 150),
        Vec2::new(120, 160),
        Vec2::new(130, 180),
    ];

    rasterizer.triangle(t0[0], t0[1], t0[2], RED.into());
    rasterizer.triangle(t1[0], t1[1], t1[2], WHITE.into());
    rasterizer.triangle(t2[0], t2[1], t2[2], GREEN.into());
    rasterizer.write_to_file("test.png");
}
