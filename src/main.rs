use std::env;
use std::process::Command;
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let output = if cfg!(target_os = "windows") {
        // Command::new("notify-send")
        //     .arg("foo bar")
        //     .output()
        //     .expect("");
        
    } else {
        Command::new("notify-send")
            .arg("foo bar")
            .output()
            .expect("");
    };
    println!("{:?}", output);
}
