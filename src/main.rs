extern crate libpulse_binding as pulse;

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
    let (app, icons) = do_mic();
    do_event_loop(app, icons);
}

fn get_pactl_data() -> String {
    let output = Command::new("/usr/bin/pactl")
        .arg("list")
        .arg("sources")
        .output()
        .expect("failed to execute process");

    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn find_mic_status(pactl_data: String) -> bool {
    for (num, line) in pactl_data.lines().enumerate() {
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

fn do_mic() -> (systray::Application, std::vec::Vec<String>) {
    let app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let icons = vec![
        "/home/iphands/prog/rust/rust_tray/assets/mic_red.png".to_string(),
        "/home/iphands/prog/rust/rust_tray/assets/mic_green.png".to_string()
    ];

    if find_mic_status(get_pactl_data()) {
        app.set_icon_from_file(&icons[1]).unwrap();
    } else {
        app.set_icon_from_file(&icons[0]).unwrap();
    }

    return (app, icons);
}

fn do_event_loop(app: systray::Application, icons: std::vec::Vec<String>) {
    let mut proplist = Proplist::new().unwrap();
    proplist.set_str(pulse::proplist::properties::APPLICATION_NAME, "FooApp").unwrap();

    let mainloop = Rc::new(RefCell::new(Mainloop::new().expect("Failed to create mainloop")));

    let context = Rc::new(RefCell::new(Context::new_with_proplist(
        mainloop.borrow().deref(),
        "FooAppContext",
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

    context.borrow_mut().set_subscribe_callback(Some(callback(app, icons)));
    context.borrow_mut().subscribe(interest, |_| {});

    mainloop.borrow_mut().run().unwrap();
    mainloop.borrow_mut().quit(Retval(0));
}

fn callback(app: systray::Application, icons: std::vec::Vec<String>) -> Box<dyn FnMut(Option<Facility>, Option<Operation>, u32)> {
    Box::new(move |facility_unsafe: Option<Facility>, operation_unsafe: Option<Operation>, _idx: u32| {
        match facility_unsafe {
            None => { eprintln!("Invalid facility received from PA"); }
            Some(_) => {
                match operation_unsafe {
                    None => { eprintln!("Invalid operation received from PA"); }
                    Some(_) => {
                        // let now = Instant::now();
                        let state = find_mic_status(get_pactl_data());
                        // println!("{}", now.elapsed().as_millis());

                        if state {
                            app.set_icon_from_file(&icons[1]).unwrap();
                        } else {
                            app.set_icon_from_file(&icons[0]).unwrap();
                        }
                    }
                }
            }
        }
    })
}
