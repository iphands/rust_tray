use clap::{Arg, App};

pub fn do_cli() -> (bool, String, String, String) {
    let matches = App::new("rust_tray")
        .version("0.2.0")
        .author("Ian Page Hands <iphands@gmail.com>")
        .about("Tray icons and feedback written in Rust")
	.arg(Arg::with_name("AUTO")
             .value_name("use auto mode")
             .short("a")
             .long("auto")
             .required(false)
             .takes_value(false)
             .help("Uses the RUNNING device found from `pactl list sources | fgrep -e Description -e State`"))
        .arg(Arg::with_name("DESC")
             .value_name("input description substring")
             .short("d")
             .long("input-description-substring")
             .required(false)
             .takes_value(true)
             .help("Description: HD Webcam C615"))
        .arg(Arg::with_name("ASSETS")
             .value_name("path to assets")
             .long("assets-path")
             .takes_value(true)
             .help("hint: ${git_repo}/assets"))
        .arg(Arg::with_name("PACTL")
             .value_name("path to pactl")
             .long("pactl-path")
             .takes_value(true)
             .help("which pactl"))
        .get_matches();

    let auto = matches.is_present("AUTO");
    let desc = matches.value_of("DESC").unwrap_or("").to_string();

    if desc == "" {
	if !auto {
	    eprintln!("Error: must choose --input-description-substring or --auto");
	    std::process::exit(1);
	}
    } else {
	if auto {
	    eprintln!("Error: cant choose --auto and --input-description-substring at the same time!");
	    std::process::exit(1);
	}
    }

    return (auto,
	    desc,
            matches.value_of("ASSETS").unwrap_or("/opt/rust_tray/assets").to_string(),
            matches.value_of("PACTL").unwrap_or("/usr/bin/pactl").to_string());
}
