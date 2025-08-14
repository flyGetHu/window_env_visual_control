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
        Box::new(|cc| {
            // 设置中文字体支持
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app::app::EnvManagerApp::new(cc)))
        }),
    );

    if let Err(e) = result {
        log::error!("Failed to start application: {}", e);
    }
}

/// 设置自定义字体以支持中文显示
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut font_loaded = false;
    
    // 添加中文字体
    // 使用系统默认的中文字体
    #[cfg(target_os = "windows")]
    {
        // Windows系统字体路径
        let font_paths = vec![
            "C:/Windows/Fonts/msyh.ttc",     // 微软雅黑
            "C:/Windows/Fonts/simsun.ttc",   // 宋体
            "C:/Windows/Fonts/simhei.ttf",   // 黑体
            "C:/Windows/Fonts/msyhl.ttc",    // 微软雅黑 Light
            "C:/Windows/Fonts/msyhbd.ttc",   // 微软雅黑 Bold
        ];
        
        for (i, font_path) in font_paths.iter().enumerate() {
            if let Ok(font_data) = std::fs::read(font_path) {
                let font_name = format!("chinese_font_{}", i);
                log::info!("Successfully loaded Chinese font: {}", font_path);
                
                fonts.font_data.insert(
                    font_name.clone(),
                    egui::FontData::from_owned(font_data),
                );
                
                // 将中文字体添加到字体族中，放在最前面以确保优先使用
                fonts.families.entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_name.clone());
                fonts.families.entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, font_name);
                
                font_loaded = true;
                break; // 只要成功加载一个字体就够了
            } else {
                log::warn!("Failed to load font: {}", font_path);
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // 非Windows系统的字体配置
        let font_paths = vec![
            "/System/Library/Fonts/PingFang.ttc",        // macOS
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc", // Linux
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",        // Linux fallback
        ];
        
        for (i, font_path) in font_paths.iter().enumerate() {
            if let Ok(font_data) = std::fs::read(font_path) {
                let font_name = format!("chinese_font_{}", i);
                log::info!("Successfully loaded Chinese font: {}", font_path);
                
                fonts.font_data.insert(
                    font_name.clone(),
                    egui::FontData::from_owned(font_data),
                );
                
                fonts.families.entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_name.clone());
                fonts.families.entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, font_name);
                
                font_loaded = true;
                break;
            } else {
                log::warn!("Failed to load font: {}", font_path);
            }
        }
    }
    
    if !font_loaded {
        log::warn!("No Chinese fonts were loaded, Chinese characters may not display correctly");
        
        // 尝试使用内嵌的Noto Sans字体作为备用
        // 这需要在Cargo.toml中添加相应的依赖
        // 现在先使用默认字体并记录警告
    }
    
    // 应用字体设置
    ctx.set_fonts(fonts);
    log::info!("Font configuration applied");
}
