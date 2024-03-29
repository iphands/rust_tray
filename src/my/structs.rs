#[derive(Debug)]
pub struct Config {
    pub auto: bool,
    pub desc_substr: String,
    pub assets_path: String,
    pub pactl_path: String,
    pub icons: Vec<String>
}
