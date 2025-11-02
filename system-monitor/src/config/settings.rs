use std::time::Duration;

pub struct Settings {
    pub update_interval: Duration,
    pub show_network: bool,
    pub show_disk: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            update_interval: Duration::from_millis(1000),
            show_network: true,
            show_disk: true,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}
