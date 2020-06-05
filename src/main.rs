#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate libpulse_binding as pulse;

mod my;

fn main() {
    let tray_app = my::gui::start_app();
    my::gui::do_mic(&tray_app);
    my::pulse::do_event_loop(tray_app);
}
