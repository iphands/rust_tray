use std::thread;
use std::time::Duration;
use std::process::Command;

fn main() -> Result<(), systray::Error> {
    return do_mic();
}

fn get_pactl_data() -> String {
    let output = Command::new("/usr/bin/pactl")
        .arg("list")
        .output()
        .expect("failed to execute process");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn find_mic_status(pactl_data: String) -> bool {
    let mut lines = pactl_data.lines();
    for (num, line) in lines.enumerate() {
        if line.contains("Description: HD Webcam C615") {
            // println!("{}: {}", num, line);
            // println!("{}", pactl_data.lines().nth(num + 5).unwrap());
            if pactl_data.lines().nth(num + 5).unwrap().contains("Mute: yes") {
                return false;
            } else {
                return true;
            }
        }
    }
    return true;
}

fn do_mic() -> Result<(), systray::Error> {
    let mut app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let icons = [
        "/home/iphands/prog/rust/rust_tray/assets/mic_red.png",
        "/home/iphands/prog/rust/rust_tray/assets/mic_green.png"
    ];

    let mut last_state = false;

    loop {
        let state = find_mic_status(get_pactl_data());

        if state != last_state {
            if state {
                app.set_icon_from_file(icons[1]);
            } else {
                app.set_icon_from_file(icons[0]);
            }
        }

        last_state = state;
        thread::sleep(Duration::from_millis(1000));
    }
}
