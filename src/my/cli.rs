use clap::{Arg, App};

use crate::my::structs;

pub fn do_cli() -> structs::Config {
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
             .help("hint: ${git_repo}/assets"))
        .arg(Arg::with_name("PACTL")
             .value_name("path to pactl")
             .short("p")
             .long("pactl-path")
             .takes_value(true)
             .help("which pactl"))
        .get_matches();

    structs::Config {
        desc_substr: matches.value_of("DESC").unwrap().to_string(),
        assets_path: matches.value_of("ASSETS").unwrap_or("/opt/rust_tray/assets").to_string(),
        pactl_path: matches.value_of("PACTL").unwrap_or("/usr/bin/pactl").to_string()
    }
}

