use std::{
    sync::Arc,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

mod app;
mod camera;

fn main() -> eframe::Result {
    // Create a shared data instance
    let shared_data = camera::SharedData::default();

    // Clone that shit up
    let camera_shared = shared_data.clone();

    // Close on gui exit
    let running = Arc::new(AtomicBool::new(true));
    let camera_running = running.clone();

    // Spwan it and get data
    thread::spawn(move || {
        if let Err(e) = camera::capture_camera_data(camera_shared, camera_running) {
            eprintln!("Camera thread failed: {e}");
        }
    });

    // This will block as egui blocks on main loop
    let result = app::start_gui(shared_data);
    running.store(false, Ordering::Relaxed);

    result
}
