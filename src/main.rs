use eframe::egui::{self, Color32, ColorImage};
use libc::printf;
use std::fs::OpenOptions;
use std::io::Write;
mod v4l2;

const DEVICE_NAME: &str = "/dev/video0";

struct WebCamUI {
    v4l2_device: v4l2::V4L2VideoDevice,
}

impl WebCamUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        // Self::default();
        let v4l2_device = v4l2::V4L2VideoDevice::new(&DEVICE_NAME);

        WebCamUI {
            v4l2_device: (v4l2_device),
        }
    }
}

impl eframe::App for WebCamUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let frame = self.v4l2_device.get_frame();
        //Yuyv
        let data = frame.data();

        // let width = frame.width();
        // let height = frame.height();
        // let mut color_data: Vec<Color32> = Vec::with_capacity(width * height);
        let color_data: Vec<Color32> = data
            .iter()
            .step_by(2)
            .map(|y| egui::Color32::from_rgb(*y, *y, *y))
            .collect();

        // let width = frame.width();
        // let height = frame.height();
        //
        // let mut color_data: Vec<Color32> = Vec::with_capacity(width * height);
        //
        //
        // // Convert YUYV to RGB
        // for chunk in data.chunks(4) {
        //     if chunk.len() != 4 {
        //         eprintln!("Chunk length is not 4: {:?}", chunk);
        //         continue; // Skip this chunk if it's not complete
        //     }
        //     let y1 = chunk[0] as i32;
        //     let u = chunk[1] as i32;
        //     let y2 = chunk[2] as i32;
        //     let v = chunk[3] as i32;
        //
        //     let r1 = (y1 as f32 + 1.402 * (v as f32 - 128.0)).clamp(0.0, 255.0) as u8;
        //     let g1 = (y1 as f32 - 0.344 * (u as f32 - 128.0) - 0.714 * (v as f32 - 128.0))
        //         .clamp(0.0, 255.0) as u8;
        //     let b1 = (y1 as f32 + 1.772 * (u as f32 - 128.0)).clamp(0.0, 255.0) as u8;
        //
        //     let r2 = (y2 as f32 + 1.402 * (v as f32 - 128.0)).clamp(0.0, 255.0) as u8;
        //     let g2 = (y2 as f32 - 0.344 * (u as f32 - 128.0) - 0.714 * (v as f32 - 128.0))
        //         .clamp(0.0, 255.0) as u8;
        //     let b2 = (y2 as f32 + 1.772 * (u as f32 - 128.0)).clamp(0.0, 255.0) as u8;
        //
        //     color_data.push(Color32::from_rgb(r1, g1, b1));
        //     color_data.push(Color32::from_rgb(r2, g2, b2));
        // }

        let image = ColorImage {
            size: [frame.width(), frame.height()],
            pixels: color_data,
        };

        let texture = ctx.load_texture("image", image, egui::TextureOptions::LINEAR);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image((texture.id(), texture.size_vec2()));
        });
    }
}

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
