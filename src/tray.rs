use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuId, MenuItem},
};

#[derive(Debug)]
pub enum TrayAction {
    OpenSettings,
    ToggleEnabled,
    Quit,
}

pub struct AppTray {
    
    
    _tray_icon: TrayIcon,

    settings_id: MenuId,
    toggle_id: MenuId,
    quit_id: MenuId,
}

impl AppTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let menu = Menu::new();

        let settings_item = MenuItem::new("Paramètres", true, None);

        let toggle_item = MenuItem::new("Activer / Désactiver", true, None);

        let quit_item = MenuItem::new("Quitter", true, None);

        menu.append(&settings_item)?;

        menu.append(&toggle_item)?;

        menu.append(&quit_item)?;

        let settings_id = settings_item.id().clone();

        let toggle_id = toggle_item.id().clone();

        let quit_id = quit_item.id().clone();

        let icon = create_default_icon()?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Hot Corners")
            .with_icon(icon)
            .build()?;

        Ok(Self {
            _tray_icon: tray_icon,

            settings_id,
            toggle_id,
            quit_id,
        })
    }

    pub fn poll_action(&self) -> Option<TrayAction> {
        let event = MenuEvent::receiver().try_recv().ok()?;

        if event.id == self.settings_id {
            return Some(TrayAction::OpenSettings);
        }

        if event.id == self.toggle_id {
            return Some(TrayAction::ToggleEnabled);
        }

        if event.id == self.quit_id {
            return Some(TrayAction::Quit);
        }

        None
    }
}

fn create_default_icon() -> Result<Icon, Box<dyn std::error::Error>> {
    const WIDTH: u32 = 32;
    const HEIGHT: u32 = 32;

    let mut rgba = Vec::with_capacity((WIDTH * HEIGHT * 4) as usize);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let border = x < 3 || y < 3 || x >= WIDTH - 3 || y >= HEIGHT - 3;

            let corner = (x < 11 && y < 11)
                || (x >= WIDTH - 11 && y < 11)
                || (x < 11 && y >= HEIGHT - 11)
                || (x >= WIDTH - 11 && y >= HEIGHT - 11);

            if border || corner {
                rgba.extend_from_slice(&[255, 255, 255, 255]);
            } else {
                rgba.extend_from_slice(&[30, 30, 30, 255]);
            }
        }
    }

    Ok(Icon::from_rgba(rgba, WIDTH, HEIGHT)?)
}
