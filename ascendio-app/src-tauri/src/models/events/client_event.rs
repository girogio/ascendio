use simconnect::DWORD;
use std::fmt::Display;
use strum::{AsRefStr, EnumIter, EnumString};

#[allow(non_camel_case_types)]
#[derive(EnumIter, EnumString, AsRefStr, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum ClientEvent {
    ENGINE_1_MASTER_SET(bool),
    ENGINE_2_MASTER_SET(bool),
    ENGINE_MODE_IGN_SET,
    ENGINE_MODE_CRANK_SET,
    ENGINE_MODE_NORM_SET,
    FLAPS_1,
    FLAPS_2,
    FLAPS_3,
    FLAPS_4,
    FLAPS_DOWN,
    FLAPS_UP,
}

impl ClientEvent {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn to_dword(&self) -> DWORD {
        match self {
            ClientEvent::ENGINE_1_MASTER_SET(set) => DWORD::from(*set),
            ClientEvent::ENGINE_2_MASTER_SET(set) => DWORD::from(*set),
            _ => DWORD::from(0 as u32),
        }
    }
}

impl Display for ClientEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_event_as_string() {
        assert_eq!(
            ClientEvent::ENGINE_1_MASTER_SET(true).as_str(),
            "ENGINE_1_MASTER_SET"
        );
        assert_eq!(
            ClientEvent::ENGINE_1_MASTER_SET(false).as_str(),
            "ENGINE_1_MASTER_SET"
        );
        assert_eq!(
            ClientEvent::ENGINE_2_MASTER_SET(true).as_str(),
            "ENGINE_2_MASTER_SET"
        );
        assert_eq!(
            ClientEvent::ENGINE_2_MASTER_SET(false).as_str(),
            "ENGINE_2_MASTER_SET"
        );
        assert_eq!(
            ClientEvent::ENGINE_MODE_IGN_SET.as_str(),
            "ENGINE_MODE_IGN_SET"
        );
        assert_eq!(
            ClientEvent::ENGINE_MODE_CRANK_SET.as_str(),
            "ENGINE_MODE_CRANK_SET"
        );
        assert_eq!(
            ClientEvent::ENGINE_MODE_NORM_SET.as_str(),
            "ENGINE_MODE_NORM_SET"
        );
        assert_eq!(ClientEvent::FLAPS_1.as_str(), "FLAPS_1");
        assert_eq!(ClientEvent::FLAPS_2.as_str(), "FLAPS_2");
        assert_eq!(ClientEvent::FLAPS_3.as_str(), "FLAPS_3");
        assert_eq!(ClientEvent::FLAPS_4.as_str(), "FLAPS_4");
        assert_eq!(ClientEvent::FLAPS_DOWN.as_str(), "FLAPS_DOWN");
        assert_eq!(ClientEvent::FLAPS_UP.as_str(), "FLAPS_UP");
    }
}
