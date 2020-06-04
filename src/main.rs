extern crate clap;
extern crate libpulse_binding as pulse;

use clap::{Arg, App};

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::process::Command;
// use std::time::Instant;

use pulse::mainloop::standard::Mainloop;
use pulse::context::Context;
use pulse::proplist::Proplist;
use pulse::mainloop::standard::IterateResult;
use pulse::def::Retval;

use pulse::context::subscribe::subscription_masks;
use pulse::context::subscribe::Operation;
use pulse::context::subscribe::Facility;

fn main() {
    let (desc, assets) = do_cli();
    let (tray_app, icons) = start_app(assets);
    do_mic(&tray_app, &icons, &desc);
    do_event_loop(tray_app, icons, desc);
}

fn do_cli() -> (String, String) {
    let matches = App::new("rust_tray")
        .version("0.1.0")
        .author("Ian Page Hands <iphands@gmail.com>")
        .about("Tray icons and feedback written in Rust")
        .arg(Arg::with_name("DESC")
             .value_name("input description substring")
             .short("d")
             .long("input-description-substring")
             .required(true)
             .takes_value(true)
             .help("Description: HD Webcam C615"))
        .arg(Arg::with_name("ASSETS")
             .value_name("path to assets")
             .short("a")
             .long("assets-path")
             .takes_value(true)
             .help("/opt/rust_tray/assets"))
        .get_matches();

    let desc = matches.value_of("DESC").unwrap().to_string();
    let assets = matches.value_of("ASSETS").unwrap_or("/opt/rust_tray/assets").to_string();

    return (desc, assets);
}

fn get_pactl_data() -> String {
    let output = Command::new("/usr/bin/pactl")
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

fn start_app(assets: String) -> (systray::Application, std::vec::Vec<String>) {
    let app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let icons = vec![
        format!("{}{}", assets, "/mic_red.png").to_string(),
        format!("{}{}", assets, "/mic_green.png").to_string()
    ];

    return (app, icons);
}

fn do_mic(app: &systray::Application, icons: &std::vec::Vec<String>, description_substring: &std::string::String) {
    // let now = Instant::now();
    let state = find_mic_status(get_pactl_data(), description_substring);
    // println!("{}", now.elapsed().as_millis());

    if state {
        app.set_icon_from_file(&icons[1]).unwrap();
        return
    }

    app.set_icon_from_file(&icons[0]).unwrap();
}

fn do_event_loop(app: systray::Application, icons: std::vec::Vec<String>, description_substring: std::string::String) {
    let mut proplist = Proplist::new().unwrap();
    proplist.set_str(pulse::proplist::properties::APPLICATION_NAME, "rust_tray").unwrap();

    let mainloop = Rc::new(RefCell::new(Mainloop::new().expect("Failed to create mainloop")));

    let context = Rc::new(RefCell::new(Context::new_with_proplist(
        mainloop.borrow().deref(),
        "RustTrayContext",
        &proplist
    ).expect("Failed to create new context")));

    context.borrow_mut().connect(None, pulse::context::flags::NOFLAGS, None).expect("Failed to connect context");

    // Wait for context to be ready
    loop {
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Quit(_) |
            IterateResult::Err(_) => {
                eprintln!("iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match context.borrow().get_state() {
            pulse::context::State::Ready => { break; },
            pulse::context::State::Failed |
            pulse::context::State::Terminated => {
                eprintln!("context state failed/terminated, quitting...");
                return;
            },
            _ => {},
        }
    }

    let interest = subscription_masks::SOURCE;

    context.borrow_mut().set_subscribe_callback(Some(callback(app, icons, description_substring)));
    context.borrow_mut().subscribe(interest, |_| {});

    mainloop.borrow_mut().run().unwrap();
    mainloop.borrow_mut().quit(Retval(0));
}

fn callback(app: systray::Application, icons: std::vec::Vec<String>, description_substring: String) -> Box<dyn FnMut(Option<Facility>, Option<Operation>, u32)> {
    Box::new(move |facility_unsafe: Option<Facility>, operation_unsafe: Option<Operation>, _idx: u32| {
        match facility_unsafe {
            None => { eprintln!("Invalid facility received from PA"); }
            Some(_) => {
                match operation_unsafe {
                    None => { eprintln!("Invalid operation received from PA"); }
                    Some(_) => {
                        do_mic(&app, &icons, &description_substring);
                    }
                }
            }
        }
    })
}
