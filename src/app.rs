use crate::camera;
use eframe::egui::{self, TextureHandle};
use std::time::Instant;

struct MyEguiApp {
    shared_data: camera::SharedData,
    texture: Option<TextureHandle>,
    last_frame_time: Instant,
    fps: f64,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, shared_data: camera::SharedData) -> Self {
        Self {
            shared_data,
            texture: None,
            last_frame_time: Instant::now(),
            fps: 0.0,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f64();
        self.last_frame_time = now;
        if dt > 0.0 {
            self.fps = self.fps * 0.9 + (1.0 / dt) * 0.1;
        }

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
                    ui.image(egui::load::SizedTexture::new(tex.id(), available));
                }
                None => {
                    ui.label("Waiting for camera...");
                }
            };
        });

        egui::Area::new(egui::Id::new("fps_overlay"))
            .fixed_pos(egui::pos2(10.0, 10.0))
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("FPS: {:.0}", self.fps))
                        .color(egui::Color32::GREEN)
                        .size(18.0)
                        .background_color(egui::Color32::from_black_alpha(180)),
                );
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
