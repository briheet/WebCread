use crate::camera;
use eframe::egui;

struct MyEguiApp {
    shared_data: camera::SharedData,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, shared_data: camera::SharedData) -> Self {
        Self { shared_data }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
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
