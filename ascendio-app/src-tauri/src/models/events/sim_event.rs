use simconnect::SimConnector;

use crate::models::ClientEvent;

#[derive(PartialEq, Eq, Debug)]
pub struct SimEvent {
    pub id: u32,
    pub client_event: ClientEvent,
}

impl SimEvent {
    pub fn new(id: u32, client_event: ClientEvent) -> Self {
        Self { id, client_event }
    }

    pub fn register(&self, conn: &mut SimConnector) {
        conn.map_client_event_to_sim_event(self.id, self.client_event.as_ref());
    }

    pub fn transmit(&self, conn: &mut SimConnector) {
        conn.transmit_client_event(
            0,
            self.id,
            self.client_event.to_dword(),
            simconnect::SIMCONNECT_GROUP_PRIORITY_HIGHEST,
            simconnect::SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
        );
    }
}
