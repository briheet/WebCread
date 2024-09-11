use eframe::egui::{self, Color32, ColorImage, TextureHandle};
use eframe::glow::Texture;
use libc::printf;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::{self, Receiver, Sender};
use v4l2::V4L2VideoDevice;
mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

struct WebCamUI {
    rx: Receiver<TextureHandle>,
    last_texture: Option<TextureHandle>,
}

impl WebCamUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        // Self::default();
        let ctx = cc.egui_ctx.clone();
        let v4l2_device = v4l2::V4L2VideoDevice::new(&DEVICE_NAME);

        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || feed_gui(v4l2_device, tx, ctx));

        WebCamUI {
            rx,
            last_texture: None,
        }
    }
}

impl eframe::App for WebCamUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        while let Ok(v) = self.rx.try_recv() {
            self.last_texture = Some(v);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.last_texture {
                ui.image((texture.id(), texture.size_vec2()));
            }
        });
    }
}

fn feed_gui(v4l2_device: v4l2::V4L2VideoDevice, tx: Sender<TextureHandle>, ctx: egui::Context) {
    loop {
        let frame = v4l2_device.get_frame();
        let data = frame.data();

        let ys = data.iter().step_by(2);
        let us = data
            .iter()
            .skip(1)
            .step_by(4)
            .flat_map(|u| std::iter::repeat(u).take(2));
        let vs = data
            .iter()
            .skip(3)
            .step_by(4)
            .flat_map(|u| std::iter::repeat(u).take(2));

        let color_data: Vec<Color32> = ys
            .zip(us)
            .zip(vs)
            .map(|((y, u), v)| {
                let y = (*y as f32) - 16.0;
                let u = (*u as f32) - 128.0;
                let v = (*v as f32) - 128.0;
                let r = 1.164 * y + 1.596 * v;
                let g = 1.164 * y - 0.392 * u - 0.813 * v;
                let b = 1.164 * y + 2.017 * u;

                egui::Color32::from_rgb(r as u8, g as u8, b as u8)
            })
            .collect();

        // let color_data: Vec<Color32> = data
        //     .iter()
        //     .step_by(2)
        //     .map(|y| egui::Color32::from_rgb(*y, *y, *y))
        //     .collect();

        let image = ColorImage {
            size: [frame.width(), frame.height()],
            pixels: color_data,
        };

        let texture = ctx.load_texture("image", image, egui::TextureOptions::LINEAR);
        tx.send(texture).unwrap();
        ctx.request_repaint();
    }
}

// Y U Y V
// Ys -> take every other Y
// Us -> take every 4th U, and duplicate
// Y U Y V

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(WebCamUI::new(cc)))),
    );

    // let mut output = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .open("test.yuv")
    //     .unwrap();
    // output.write_all(frame.data()).unwrap();
}
