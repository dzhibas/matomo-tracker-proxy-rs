#[derive(Clone, Default, Debug)]
pub struct Config {
    pub matomo_url: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}
