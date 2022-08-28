use tinyrenderer::math::Vec2;
use tinyrenderer::model::Model;
use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::RGB;

#[test]
fn test_wire_renderer() {
    let width = 800;
    let height = 800;
    let mut rasterizer = Rasterizer::new(width, height);

    let model = Model::from("model/african_head.obj").unwrap();
    const WHITE: RGB = RGB {
        r: 255,
        g: 255,
        b: 255,
    };
    for i in 0..model.num_faces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vertex(face[j]);
            let v1 = model.vertex(face[(j + 1) % 3]);
            let x0 = ((v0.x + 1.) * (width as f32) / 2.) as isize;
            let y0 = ((v0.y + 1.) * (height as f32) / 2.) as isize;
            let x1 = ((v1.x + 1.) * (width as f32) / 2.) as isize;
            let y1 = ((v1.y + 1.) * (height as f32) / 2.) as isize;
            rasterizer.line(Vec2::new(x0, y0), Vec2::new(x1, y1), WHITE)
        }
    }

    rasterizer.write_to_file("test.tga");
}
