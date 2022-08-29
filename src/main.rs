#![windows_subsystem = "windows"]

use eframe::egui;
use tinyrenderer::egui_window::MyApp;
use tinyrenderer::math::Vec2;
use tinyrenderer::model::Model;
use tinyrenderer::rasterizer::Rasterizer;
use tinyrenderer::tga::WHITE;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;
    let app = MyApp::new(|app| {

        // start drawing
        let mut rasterizer = Rasterizer::new(WIDTH, HEIGHT);

        let model = Model::from("model/african_head.obj").unwrap();

        for i in 0..model.num_faces() {
            let face = model.face(i);
            for j in 0..3 {
                let v0 = model.vertex(face[j]);
                let v1 = model.vertex(face[(j + 1) % 3]); // 三角形内的下一个顶点

                // 只要 xy 不要 z
                let x0 = ((v0.x + 1.) * (WIDTH as f32) / 2.) as isize;
                let y0 = ((v0.y + 1.) * (HEIGHT as f32) / 2.) as isize;
                let x1 = ((v1.x + 1.) * (WIDTH as f32) / 2.) as isize;
                let y1 = ((v1.y + 1.) * (HEIGHT as f32) / 2.) as isize;
                rasterizer.line(Vec2::new(x0, y0), Vec2::new(x1, y1), WHITE.into())
            }
        }

        // end drawing
        app.update_image(rasterizer.image);
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    eframe::run_native("rust-tiny-renderer", options, Box::new(|_cc| Box::new(app)));
}
