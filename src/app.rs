use crate::camera;
use eframe::egui::{self, TextureHandle};

struct MyEguiApp {
    shared_data: camera::SharedData,
    texture: Option<TextureHandle>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, shared_data: camera::SharedData) -> Self {
        Self {
            shared_data,
            texture: None,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(frame) = self.shared_data.lock().unwrap().take() {
            let image = egui::ColorImage::from_rgb(
                [frame.width as usize, frame.height as usize],
                &frame.data,
            );

            match &mut self.texture {
                Some(tex) => tex.set(image, egui::TextureOptions::LINEAR),
                None => {
                    self.texture =
                        Some(ctx.load_texture("webcam", image, egui::TextureOptions::LINEAR));
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.texture {
                Some(tex) => {
                    let available = ui.available_size();
                    ui.image(egui::load::SizedTexture::new(
                        tex.id(),
                        available,
                    ));
                }
                None => {
                    ui.label("Waiting for camera...");
                }
            };
        });

        ctx.request_repaint();
    }
}

pub fn start_gui(shared_data: camera::SharedData) -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "WebCread",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc, shared_data)))),
    )
}
