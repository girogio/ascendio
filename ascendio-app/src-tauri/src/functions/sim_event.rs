use simconnect::SimConnector;

struct Input {
    event: String, // The event to be triggered i.e.
    input_id: u32, // The id we use to trigger the event
}

#[tauri::command]
pub fn send_event(event_name: &str) -> String {
    let mut conn = SimConnector::new();
    let is_conn = conn.connect("ascendio");

    let custom_input = Input {
        event: event_name.to_string(),
        input_id: 3,
    };

    conn.map_client_event_to_sim_event(custom_input.input_id, custom_input.event.as_str());

    conn.transmit_client_event(
        0,
        custom_input.input_id,
        0 as u32,
        simconnect::SIMCONNECT_GROUP_PRIORITY_HIGHEST,
        simconnect::SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
    );

    format!("Hey {event_name}, SimConnect connected: {}", is_conn)
}
