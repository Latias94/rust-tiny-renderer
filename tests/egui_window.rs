#![windows_subsystem = "windows"]

use eframe::egui;
use tinyrenderer::egui_window::MyApp;
use tinyrenderer::math::{Vec2, Vec3};
use tinyrenderer::model::Model;
use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::RGBA;

#[test]
fn test_window() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let app = MyApp::new(|app| {
        let mut rasterizer = Rasterizer::new(WIDTH, HEIGHT);
        let light_dir = Vec3::new(0f32, 0f32, -1f32);

        let model = Model::from("model/african_head.obj").unwrap();
        for i in 0..model.num_faces() {
            let face = model.face(i);
            let mut screen_coords = vec![Vec2::default(); 3];
            let mut world_corrds = vec![Vec3::default(); 3];
            for j in 0..3 {
                let vertex = *model.vertex(face[j]);
                let x = ((vertex.x + 1.) * (WIDTH as f32) / 2.) as isize;
                let y = ((vertex.y + 1.) * (HEIGHT as f32) / 2.) as isize;
                screen_coords[j] = Vec2::new(x, y);
                world_corrds[j] = vertex;
            }
            let vector0 = world_corrds[2] - world_corrds[0];
            let vector1 = world_corrds[1] - world_corrds[0];
            let mut normal = vector0.cross_product(vector1);
            normal = normal.normalize();
            let intensity = normal * light_dir;
            if intensity > 0_f32 {
                let gray_scale = (intensity * 255_f32) as u8;
                rasterizer.triangle(
                    screen_coords[0],
                    screen_coords[1],
                    screen_coords[2],
                    RGBA {
                        r: gray_scale,
                        g: gray_scale,
                        b: gray_scale,
                        a: 255,
                    }
                    .into(),
                )
            }
        }

        app.update_image(rasterizer.image);
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    eframe::run_native("rust-tiny-renderer", options, Box::new(|_cc| Box::new(app)));
}
