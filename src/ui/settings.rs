use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use eframe::egui;

use crate::{
    config::{Action, Config, HotCorner},
    tray::{AppTray, TrayAction},
};

pub struct SettingsApp {
    
    config: Arc<Mutex<Config>>,

    
    enabled: Arc<AtomicBool>,

    
    tray: AppTray,

    
    status_message: String,

    
    
    
    
    
    
    
    really_quit: bool,
}

impl SettingsApp {
    pub fn new(config: Arc<Mutex<Config>>, enabled: Arc<AtomicBool>, tray: AppTray) -> Self {
        Self {
            config,
            enabled,
            tray,
            status_message: String::new(),
            really_quit: false,
        }
    }

    
    
    

    fn corner_ui(ui: &mut egui::Ui, title: &str, id: &str, corner: &mut HotCorner) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());

            
            
            

            ui.horizontal(|ui| {
                ui.strong(title);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.checkbox(&mut corner.enabled, "Activé");
                });
            });

            ui.add_space(5.0);

            
            
            

            ui.add_enabled_ui(corner.enabled, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Action");

                    egui::ComboBox::from_id_salt(id)
                        .width(240.0)
                        .selected_text(action_name(&corner.action))
                        .show_ui(ui, |ui| {
                            action_button(ui, &mut corner.action, Action::None, "Aucune");

                            action_button(
                                ui,
                                &mut corner.action,
                                Action::ShowDesktop,
                                "Afficher le bureau",
                            );

                            action_button(
                                ui,
                                &mut corner.action,
                                Action::TaskView,
                                "Vue des tâches (Win + Tab)",
                            );

                            action_button(
                                ui,
                                &mut corner.action,
                                Action::LockScreen,
                                "Verrouiller Windows",
                            );

                            action_button(
                                ui,
                                &mut corner.action,
                                Action::OpenApplication(String::new()),
                                "Ouvrir une application",
                            );
                        });
                });

                
                
                

                if let Action::OpenApplication(path) = &mut corner.action {
                    ui.add_space(6.0);

                    ui.label("Chemin de l'application");

                    ui.add(
                        egui::TextEdit::singleline(path)
                            .desired_width(f32::INFINITY)
                            .hint_text(r"C:\Program Files\...\application.exe"),
                    );
                }
            });
        });
    }

    
    
    

    fn handle_tray(&mut self, ctx: &egui::Context) {
        
        
        while let Some(action) = self.tray.poll_action() {
            match action {
                
                
                
                TrayAction::OpenSettings => {
                    
                    
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));

                    
                    
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));

                    
                    
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                }

                
                
                
                TrayAction::ToggleEnabled => {
                    let was_enabled = self.enabled.fetch_xor(true, Ordering::SeqCst);

                    self.status_message = if was_enabled {
                        "Hot Corners désactivés.".to_string()
                    } else {
                        "Hot Corners activés.".to_string()
                    };
                }

                
                
                
                TrayAction::Quit => {
                    
                    self.really_quit = true;

                    
                    
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }
}





impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        
        
        
        
        
        
        

        ctx.request_repaint_after(Duration::from_millis(100));

        self.handle_tray(ctx);

        
        
        

        let close_requested = ctx.input(|input| input.viewport().close_requested());

        
        
        
        
        
        
        
        
        

        if close_requested && !self.really_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);

            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));

            return;
        }

        
        
        

        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing = egui::vec2(10.0, 10.0);

        style.spacing.button_padding = egui::vec2(12.0, 7.0);

        ctx.set_style(style);

        
        
        

        let mut config = match self.config.lock() {
            Ok(config) => config,

            Err(error) => {
                eprintln!("Erreur Mutex : {error}");

                return;
            }
        };

        let enabled = self.enabled.load(Ordering::SeqCst);

        
        
        

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(5.0);

            
            
            

            ui.horizontal(|ui| {
                ui.heading("Hot Corners");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if enabled {
                        ui.strong("● Actif");
                    } else {
                        ui.label("○ Désactivé");
                    }
                });
            });

            ui.label("Configure le comportement des quatre coins de ton écran.");

            ui.add_space(5.0);

            ui.separator();

            ui.add_space(10.0);

            
            
            

            Self::corner_ui(
                ui,
                "↖  Haut gauche",
                "top_left_action",
                &mut config.top_left,
            );

            ui.add_space(8.0);

            
            
            

            Self::corner_ui(
                ui,
                "↗  Haut droit",
                "top_right_action",
                &mut config.top_right,
            );

            ui.add_space(8.0);

            
            
            

            Self::corner_ui(
                ui,
                "↙  Bas gauche",
                "bottom_left_action",
                &mut config.bottom_left,
            );

            ui.add_space(8.0);

            
            
            

            Self::corner_ui(
                ui,
                "↘  Bas droit",
                "bottom_right_action",
                &mut config.bottom_right,
            );

            ui.add_space(15.0);

            ui.separator();

            ui.add_space(10.0);

            
            
            

            ui.heading("Détection");

            ui.label("Ajuste la sensibilité des coins.");

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Zone sensible");

                ui.add(egui::Slider::new(&mut config.corner_size, 1..=50).suffix(" px"));
            });

            ui.horizontal(|ui| {
                ui.label("Délai");

                ui.add(egui::Slider::new(&mut config.activation_delay_ms, 0..=2000).suffix(" ms"));
            });

            ui.add_space(15.0);

            ui.separator();

            ui.add_space(10.0);

            
            
            

            ui.horizontal(|ui| {
                let toggle_text = if enabled {
                    "Désactiver temporairement"
                } else {
                    "Activer Hot Corners"
                };

                
                
                

                if ui.button(toggle_text).clicked() {
                    self.enabled.store(!enabled, Ordering::SeqCst);

                    self.status_message = if enabled {
                        "Hot Corners désactivés.".to_string()
                    } else {
                        "Hot Corners activés.".to_string()
                    };
                }

                
                
                

                if ui.button("Sauvegarder").clicked() {
                    match config.save() {
                        Ok(_) => {
                            self.status_message = "Configuration sauvegardée.".to_string();
                        }

                        Err(error) => {
                            self.status_message = format!("Erreur : {error}");
                        }
                    }
                }
            });

            
            
            

            if !self.status_message.is_empty() {
                ui.add_space(8.0);

                ui.label(&self.status_message);
            }
        });
    }
}





fn action_button(ui: &mut egui::Ui, current: &mut Action, action: Action, label: &str) {
    let selected = std::mem::discriminant(current) == std::mem::discriminant(&action);

    if ui.selectable_label(selected, label).clicked() {
        *current = action;
    }
}





fn action_name(action: &Action) -> &'static str {
    match action {
        Action::None => "Aucune",

        Action::ShowDesktop => "Afficher le bureau",

        Action::TaskView => "Vue des tâches (Win + Tab)",

        Action::LockScreen => "Verrouiller Windows",

        Action::OpenApplication(_) => "Ouvrir une application",
    }
}
