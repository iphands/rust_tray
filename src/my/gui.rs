use std::process::Command;

use crate::my::global::*;

pub fn start_app() -> systray::Application {
    let app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    return app;
}

pub fn do_mic(app: &systray::Application) {
    let state = match &CONFIG.auto {
	true => find_mic_status_by_auto(get_pactl_data()),
	false => find_mic_status_by_desc(get_pactl_data())
    };

    if state {
        app.set_icon_from_file(&CONFIG.icons[1]).unwrap();
        return
    }

    app.set_icon_from_file(&CONFIG.icons[0]).unwrap();

}

fn get_pactl_data() -> String {
    let output = Command::new(CONFIG.pactl_path.clone())
        .arg("list")
        .arg("sources")
        .output()
        .expect("failed to execute process");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn check_mute_line(offset: usize, mut lines: std::str::Lines) -> bool {
    let line = lines.nth(offset).unwrap();
    assert!(line.contains("Mute: "), "Expected to find a line beginnig with \"Mute:\", instead found {}", line);
    return line.contains("Mute: yes");
}

fn check_vol_zero(offset: usize, mut lines: std::str::Lines) -> bool {
    let line = lines.nth(offset).unwrap();
    assert!(line.contains("Volume: "), "Expected to find a line beginnig with \"Volume:\", instead found {}", line);
    return line.contains(" /   0% / ");
}

fn is_monitor(offset: usize, mut lines: std::str::Lines) -> bool {
    let line = lines.nth(offset).unwrap();
    assert!(line.contains("Description: "), "Expected to find a line beginnig with \"Description: \", instead found {}", line);
    return line.contains("Description: Monitor of ");
}

fn find_mic_status_by_desc(pactl_data: String) -> bool {
    assert!(CONFIG.desc_substr != "", "desc_substr is \"\" this should be impossible");
    assert!(CONFIG.auto == false, "status_by_desc called while CONFIG.auto is true");

    for (num, line) in pactl_data.lines().enumerate() {
        if line.contains("Description: ") && line.contains(&CONFIG.desc_substr) {
	    if is_monitor(num, pactl_data.lines()) {
		continue;
	    }

	    let muted = check_mute_line(num + 5, pactl_data.lines());
	    let vol_zero = check_vol_zero(num + 6, pactl_data.lines());

	    if muted || vol_zero {
		return false;
	    }

	    return true;
        }
    }

    eprintln!("Fatal error:");
    eprintln!("  Unable to find input-description-substring ({}) in pactl output!", CONFIG.desc_substr);
    eprintln!("  Please make sure that the sub string you are looking for shows up in:");
    eprintln!("  `pactl list sources | fgrep Description`");
    std::process::exit(1);
}

fn find_mic_status_by_auto(pactl_data: String) -> bool {
    assert!(CONFIG.desc_substr == "", "found input description in auto");
    assert!(CONFIG.auto == true, "status_by_auto called while CONFIG.auto is false");

    for (num, line) in pactl_data.lines().enumerate() {
        if line.contains("State: ") && line.contains("RUNNING") {
	    if is_monitor(num + 2, pactl_data.lines()) {
		continue;
	    }

	    let muted = check_mute_line(num + 7, pactl_data.lines());
	    let vol_zero = check_vol_zero(num + 8, pactl_data.lines());
	    if muted || vol_zero {
		return false;
	    }
        }
    }

    return true;
}
