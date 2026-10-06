use std::process::Command;

use windows::Win32::{
    System::Shutdown::LockWorkStation,
    UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY,
        VK_LWIN,
    },
};

use crate::config::Action;

pub fn execute_action(action: &Action) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        Action::None => {}

        Action::ShowDesktop => {
            
            send_shortcut(VK_LWIN, VIRTUAL_KEY(0x44))?;
        }

        Action::TaskView => {
            
            send_shortcut(VK_LWIN, VIRTUAL_KEY(0x09))?;
        }

        Action::LockScreen => {
            lock_screen()?;
        }

        Action::OpenApplication(path) => {
            open_application(path)?;
        }
    }

    Ok(())
}

fn send_shortcut(
    modifier: VIRTUAL_KEY,
    key: VIRTUAL_KEY,
) -> Result<(), Box<dyn std::error::Error>> {
    let inputs = [
        keyboard_input(modifier, false),
        keyboard_input(key, false),
        keyboard_input(key, true),
        keyboard_input(modifier, true),
    ];

    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };

    if sent != inputs.len() as u32 {
        return Err(std::io::Error::last_os_error().into());
    }

    Ok(())
}

fn keyboard_input(key: VIRTUAL_KEY, key_up: bool) -> INPUT {
    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        Default::default()
    };

    INPUT {
        r#type: INPUT_KEYBOARD,

        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn lock_screen() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        LockWorkStation()?;
    }

    Ok(())
}

fn open_application(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.trim();

    if path.is_empty() {
        return Err("Le chemin de l'application est vide.".into());
    }

    Command::new(path).spawn()?;

    Ok(())
}
