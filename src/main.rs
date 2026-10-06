#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod action;
mod config;
mod hotcorner;
mod tray;
mod ui;

use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use config::Config;
use hotcorner::HotCornerEngine;
use tray::AppTray;
use ui::settings::SettingsApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    
    

    let initial_config = Config::load()?;

    let shared_config = Arc::new(Mutex::new(initial_config));

    
    let enabled = Arc::new(AtomicBool::new(true));

    
    
    

    let engine_config = Arc::clone(&shared_config);

    let engine_enabled = Arc::clone(&enabled);

    thread::spawn(move || {
        let initial_engine_config = {
            let config = engine_config
                .lock()
                .expect("Impossible de verrouiller la configuration");

            config.clone()
        };

        let mut engine = HotCornerEngine::new(initial_engine_config);

        loop {
            
            if !engine_enabled.load(Ordering::SeqCst) {
                engine.reset();

                thread::sleep(Duration::from_millis(50));

                continue;
            }

            
            
            let current_config = {
                let config = engine_config
                    .lock()
                    .expect("Impossible de verrouiller la configuration");

                config.clone()
            };

            engine.set_config(current_config);

            
            if let Err(error) = engine.update() {
                eprintln!("Erreur HotCornerEngine : {error}");
            }

            
            thread::sleep(Duration::from_millis(25));
        }
    });

    
    
    

    let tray = AppTray::new()?;

    
    
    

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Hot Corners")
            .with_inner_size([620.0, 720.0])
            .with_min_inner_size([520.0, 600.0]),

        ..Default::default()
    };

    
    
    

    eframe::run_native(
        "Hot Corners",
        options,
        Box::new(move |_cc| Ok(Box::new(SettingsApp::new(shared_config, enabled, tray)))),
    )
    .map_err(|error| Box::<dyn std::error::Error>::from(error.to_string()))?;

    Ok(())
}
