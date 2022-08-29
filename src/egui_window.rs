use eframe::egui::Context;
use eframe::{egui, App, Frame};
use egui_extras::RetainedImage;
use image::{EncodableLayout, ImageBuffer, ImageOutputFormat, Rgba};
use std::io::{Cursor, Read, Seek, SeekFrom};

type Callback = fn(&mut MyApp);
pub struct MyApp {
    pub image: RetainedImage,
    callback: Callback,
}

impl MyApp {
    pub fn new(callback: Callback) -> Self {
        Self {
            image: get_retained_image(ImageBuffer::new(1, 1)),
            callback,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("This is an image:");
            self.process_events();
            self.image.show(ui)
        });
    }
}

impl MyApp {
    pub fn set_callback(&mut self, c: Callback) {
        self.callback = c;
    }

    fn process_events(&mut self) {
        (self.callback)(self);
    }

    pub fn update_image(&mut self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) {
        self.image = get_retained_image(image);
    }
}

pub fn get_retained_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> RetainedImage {
    // Create fake "file"
    let mut c = Cursor::new(Vec::new());
    image
        .write_to(&mut c, ImageOutputFormat::Png)
        .expect("imagebuffer write_to fail");
    // Write into the "file" and seek to the beginning
    c.seek(SeekFrom::Start(0)).unwrap();

    // Read the "file's" contents into a vector
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    RetainedImage::from_image_bytes("png bytes", out.as_bytes()).unwrap()
}
