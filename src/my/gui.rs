use std::process::Command;

use crate::my::global::*;

pub fn start_app() -> (systray::Application, std::vec::Vec<String>) {
    let app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let icons = vec![
        format!("{}{}", CONFIG.assets_path, "/mic_red.png").to_string(),
        format!("{}{}", CONFIG.assets_path, "/mic_green.png").to_string()
    ];

    return (app, icons);
}

pub fn do_mic(app: &systray::Application, icons: &std::vec::Vec<String>) {
    // let now = Instant::now();
    let state = find_mic_status(get_pactl_data(), &CONFIG.desc_substr);
    // println!("{}", now.elapsed().as_millis());

    if state {
        app.set_icon_from_file(&icons[1]).unwrap();
        return
    }

    app.set_icon_from_file(&icons[0]).unwrap();
}

fn get_pactl_data() -> String {
    let output = Command::new(CONFIG.pactl_path.clone())
        .arg("list")
        .arg("sources")
        .output()
        .expect("failed to execute process");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn find_mic_status(pactl_data: String, description_substring: &String) -> bool {
    for (num, line) in pactl_data.lines().enumerate() {
        if line.contains("Description: ") && line.contains(description_substring) {
            // println!("{}: {}", num, line);
            // println!("{}", pactl_data.lines().nth(num + 5).unwrap());
            if pactl_data.lines().nth(num + 5).unwrap().contains("Mute: yes") {
                return false;
            } else {
                return true;
            }
        }
    }

    eprintln!("Fatal error:");
    eprintln!("  Unable to find input-description-substring ({}) in pactl output!", description_substring);
    eprintln!("  Please make sure that the sub string you are looking for shows up in:");
    eprintln!("  `pactl list sources | fgrep Description`");
    std::process::exit(1);
}
