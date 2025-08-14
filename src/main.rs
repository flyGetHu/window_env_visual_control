mod models;
mod core;
mod utils;
mod app;

use eframe::egui;
use utils::logger;

fn main() {
    // 初始化日志系统
    logger::init_logger();
    
    log::info!("Starting Windows Environment Variables Manager");
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Windows Environment Variables Manager"),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Windows Environment Variables Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(app::app::EnvManagerApp::new(cc)))),
    );

    if let Err(e) = result {
        log::error!("Failed to start application: {}", e);
    }
}
