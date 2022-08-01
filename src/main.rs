use std::env;
use std::process::Command;
use windows::{
    core::*, UI::Notifications::*,
    Data::Xml::Dom::*
};

fn main() -> windows::core::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");

    // Windows notification
    let output = if cfg!(target_os = "windows") {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);

        // Get a toast XML template
        let toast_xml = ToastNotificationManager::GetTemplateContent(
            ToastTemplateType::ToastText02
        ).unwrap();

        // Fill in the text elements
        let toast_text_elements = toast_xml.GetElementsByTagName(
            &HSTRING::from("text")
        ).unwrap();

        toast_text_elements
            .Item(0).unwrap()
            .AppendChild(
                &toast_xml
                .CreateTextNode(&HSTRING::from("Hello from Rust!")).unwrap()
                .cast::<IXmlNode>().unwrap()
            ).unwrap();
        
        toast_text_elements
            .Item(1).unwrap()
            .AppendChild(
                &toast_xml
                .CreateTextNode(&HSTRING::from("This is some more text.")).unwrap()
                .cast::<IXmlNode>().unwrap()
            ).unwrap();

        // Create the toast and attach event listeners
        let toast = ToastNotification::CreateToastNotification(&toast_xml).unwrap();

        // Show the toast. Use PowerShell's App ID to circumvent the need to register one (this is only an example!).
        let result =
            ToastNotificationManager::CreateToastNotifierWithId(
                &HSTRING::from("{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe")
            ).unwrap()
            .Show(&toast).unwrap();
        
        dbg!(result);
    }

    else if cfg!(target_os = "macos") {
        println!("Hi mac");
    }
    
    // Linux
    else if cfg!(target_os = "linux") {

        // KDE?

        // Gnome
        Command::new("notify-send")
            .arg("foo bar")
            .output()
            .expect("");
    }
    
    else {
        println!("Not supported");
    };

    println!("{:?}", output);

    Ok(())
}
