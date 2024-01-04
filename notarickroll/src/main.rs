// notarickroll/src/main.rs
use libmal;  // Import the libmal library
use std::{thread, time::Duration};



fn simulate_conversation() {
    let conversation = vec![
        ("Unknown", "Hi!"),
        ("You", "Who are you?"),
        ("Unknown", "A Unknown Guy."),
        ("You", "Hmm. Okay. What do u want?"),
        ("Unknown", "YOUR PC!!!"),
        ("You", "WAIT!!! TELL ME WHY I CANT USE MY KEYBOARD?!?"),
        ("Unknown", "We want your PC but dont worry we turn it on later."),
        ("Unknown", "Just give us ur Password and we turn it on"),
        ("You", "Ok but pls dont hack my PC."),
        ("Unknown", "Ok ur keyboard is on. Enjoy your freetime!"),
    ];

    for (speaker, message) in conversation {
        println!("{}: {}", speaker, message);
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    if !libmal::is_root() {
        println!("Please run as root/sudo.");
        return;
    }

    let user_platform = libmal::get_user_platform();
    match user_platform.as_str() {
        "GNU/Linux X11" => {
            libmal::install_tools_linux();
            libmal::disable_keyboard_x11();
            simulate_conversation();
            libmal::enable_keyboard_x11();
        }
        "GNU/Linux Wayland" => {
            libmal::disable_keyboard_wayland();
            simulate_conversation();
            libmal::enable_keyboard_wayland();
        }
        _ => {
            println!("This script is not supported on {} platform.", user_platform);
        }
    }
}
