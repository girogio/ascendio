use log::info;
use std::collections::HashMap;

use strum::IntoEnumIterator;

use super::{ClientEvent, SimEvent};

#[derive(Debug)]
pub struct EventRegistry {
    pub events: HashMap<ClientEvent, SimEvent>,
}

impl EventRegistry {
    pub fn new() -> Self {
        let mut idx = 0;

        let events: HashMap<ClientEvent, SimEvent> =
            HashMap::from_iter(ClientEvent::iter().map(|client_event| {
                let id = idx;
                let sim_event = SimEvent::new(id, client_event);

                info!("Creating index {} for event: {}", id, client_event);

                idx += 1;

                (client_event, sim_event)
            }));

        Self { events }
    }
}
