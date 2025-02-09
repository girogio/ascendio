pub struct AppConfig {
    pub baud_rate: u32,
}

impl AppConfig {
    pub fn new() -> Self {
        Self { baud_rate: 115200 }
    }
}
