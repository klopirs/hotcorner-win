use std::time::{Duration, Instant};

use windows::Win32::{
    Foundation::{POINT, RECT},
    Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint},
    UI::WindowsAndMessaging::GetCursorPos,
};

use crate::{
    action,
    config::{Action, Config, Corner},
};

#[derive(Debug, Clone, Copy)]
pub struct Monitor {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub fn get_cursor_position() -> Result<POINT, windows::core::Error> {
    let mut point = POINT::default();

    unsafe {
        GetCursorPos(&mut point)?;
    }

    Ok(point)
}

pub fn get_monitor_from_point(point: POINT) -> Result<Monitor, windows::core::Error> {
    let monitor_handle = unsafe { MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST) };

    let mut monitor_info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,

        ..Default::default()
    };

    unsafe {
        GetMonitorInfoW(monitor_handle, &mut monitor_info).ok()?;
    }

    let rect: RECT = monitor_info.rcMonitor;

    Ok(Monitor {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    })
}

pub fn detect_corner(point: POINT, monitor: Monitor, corner_size: u32) -> Option<Corner> {
    let size = corner_size as i32;

    let near_left = point.x >= monitor.left && point.x < monitor.left + size;

    let near_right = point.x < monitor.right && point.x >= monitor.right - size;

    let near_top = point.y >= monitor.top && point.y < monitor.top + size;

    let near_bottom = point.y < monitor.bottom && point.y >= monitor.bottom - size;

    if near_left && near_top {
        return Some(Corner::TopLeft);
    }

    if near_right && near_top {
        return Some(Corner::TopRight);
    }

    if near_left && near_bottom {
        return Some(Corner::BottomLeft);
    }

    if near_right && near_bottom {
        return Some(Corner::BottomRight);
    }

    None
}

pub struct HotCornerEngine {
    config: Config,

    active_corner: Option<Corner>,

    entered_at: Option<Instant>,

    triggered: bool,
}

impl HotCornerEngine {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            active_corner: None,
            entered_at: None,
            triggered: false,
        }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cursor = get_cursor_position()?;

        let monitor = get_monitor_from_point(cursor)?;

        let detected_corner = detect_corner(cursor, monitor, self.config.corner_size);

        match detected_corner {
            Some(corner) => {
                self.handle_corner(corner)?;
            }

            None => {
                self.reset();
            }
        }

        Ok(())
    }

    fn handle_corner(&mut self, corner: Corner) -> Result<(), Box<dyn std::error::Error>> {
        
        if !self.is_same_corner(corner) {
            self.active_corner = Some(corner);

            self.entered_at = Some(Instant::now());

            self.triggered = false;

            return Ok(());
        }

        
        
        if self.triggered {
            return Ok(());
        }

        let Some(entered_at) = self.entered_at else {
            return Ok(());
        };

        let required_delay = Duration::from_millis(self.config.activation_delay_ms);

        if entered_at.elapsed() < required_delay {
            return Ok(());
        }

        if let Some(action) = self.get_action(corner).cloned() {
            println!("Déclenchement : {:?} -> {:?}", corner, action);

            action::execute_action(&action)?;

            self.triggered = true;
        }

        Ok(())
    }

    fn is_same_corner(&self, corner: Corner) -> bool {
        self.active_corner == Some(corner)
    }

    fn get_action(&self, corner: Corner) -> Option<&Action> {
        let hot_corner = match corner {
            Corner::TopLeft => &self.config.top_left,

            Corner::TopRight => &self.config.top_right,

            Corner::BottomLeft => &self.config.bottom_left,

            Corner::BottomRight => &self.config.bottom_right,
        };

        if hot_corner.enabled {
            Some(&hot_corner.action)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.active_corner = None;

        self.entered_at = None;

        self.triggered = false;
    }
}
