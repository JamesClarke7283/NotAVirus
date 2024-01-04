use os_info;
use std::{env, process::Command};
use whoami;

pub fn get_user_platform() -> String {
    // ... existing platform detection logic
}

pub fn is_root() -> bool {
    // ... existing root check logic
}

#[cfg(target_os = "linux")]
pub fn install_tools_linux() {
    let info = os_info::get();
    // ... existing logic to install tools based on Linux distro
}

pub fn disable_keyboard_x11() {
    // ... existing logic to disable keyboard for X11
}

pub fn enable_keyboard_x11() {
    // ... existing logic to enable keyboard for X11
}

pub fn disable_keyboard_wayland() {
    // ... Wayland logic
}

pub fn enable_keyboard_wayland() {
    // ... Wayland logic
}
