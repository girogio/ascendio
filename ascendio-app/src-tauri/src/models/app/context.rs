use crate::models::hardware::MCU;

use crate::models::EventRegistry;

use super::config::AppConfig;

pub struct AppContext {
    pub config: AppConfig,
    pub mcu: MCU,
    pub event_registry: EventRegistry,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            config: AppConfig::new(),
            mcu: MCU::new(),
            event_registry: EventRegistry::new(),
        }
    }
}
