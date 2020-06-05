use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use pulse::context::Context;
use pulse::context::subscribe::{subscription_masks, Operation, Facility};
use pulse::def::Retval;
use pulse::mainloop::standard::{Mainloop, IterateResult};
use pulse::proplist::Proplist;

use crate::my::gui;

pub fn do_event_loop(app: systray::Application, icons: std::vec::Vec<String>) {
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
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                eprintln!("iterate state was not success, quitting...");
                return;
            },
            IterateResult::Success(_) => {},
        }
        match context.borrow().get_state() {
            pulse::context::State::Ready => { break; },
            pulse::context::State::Failed | pulse::context::State::Terminated => {
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
        if let (Some(_fac), Some(_op)) = (facility_unsafe, operation_unsafe) {
            gui::do_mic(&app, &icons);
        }
    })
}
