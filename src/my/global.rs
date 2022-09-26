use crate::my::cli;
use crate::my::structs::Config;

lazy_static! {
    pub static ref CONFIG: Config = {
        let (auto, desc_substr, assets_path, pactl_path) = cli::do_cli();

        // setup icons
        let icons = vec![
            format!("{}{}", assets_path, "/mic_red.png").to_string(),
            format!("{}{}", assets_path, "/mic_green.png").to_string()
        ];

        return Config {
	    auto: auto,
            desc_substr: desc_substr,
            assets_path: assets_path,
            pactl_path: pactl_path,
            icons: icons,
        };
    };
}
