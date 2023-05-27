use core::time;
use std::{mem::size_of, thread};

use clap::Parser;
use widestring::Utf16String;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_UNICODE, VIRTUAL_KEY,
};

#[derive(Parser)]
#[command(version)]
struct CommandLineArgs {
    /// Delay (in seconds) until the keyboard events are sent
    #[arg(short, long, default_value_t = 3)]
    delay: u32,

    /// String of text to turned into simulated keystrokes
    #[arg(short, long)]
    text: String,
}

fn main() {
    let args = CommandLineArgs::parse();

    println!(
        "Waiting {} seconds to simulate: \"{}\"",
        args.delay, args.text
    );

    let utf16_text = Utf16String::from_str(&args.text);

    let input_struct: INPUT = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT::default(),
        },
    };

    let mut input_struct_array: [INPUT; 1] = [input_struct];

    thread::sleep(time::Duration::from_secs(args.delay.into()));

    for key in utf16_text.as_slice() {
        input_struct_array[0].Anonymous.ki.wVk = VIRTUAL_KEY(0);
        input_struct_array[0].Anonymous.ki.wScan = *key;
        input_struct_array[0].Anonymous.ki.dwFlags = KEYEVENTF_UNICODE;
        input_struct_array[0].Anonymous.ki.time = 0;
        input_struct_array[0].Anonymous.ki.dwExtraInfo = 0;

        unsafe {
            SendInput(&input_struct_array, size_of::<INPUT>() as i32);
        }

        thread::sleep(time::Duration::from_millis(34));
    }
}
