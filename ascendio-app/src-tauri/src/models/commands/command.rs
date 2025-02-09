use strum::{EnumDiscriminants, FromRepr};

#[derive(Copy, Clone, Debug, FromRepr, EnumDiscriminants, Eq, PartialEq)]
pub enum Command {
    Unknown,
    Init,
    InitAck,
    Health,
    HealthAck,
}

impl Command {
    pub fn id(&self) -> u8 {
        CommandDiscriminants::from(self) as u8
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Unknown | Self::Init | Self::InitAck | Self::Health | Self::HealthAck => {
                vec![self.id()]
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let command_type = Command::from_repr(bytes[0] as usize);

        let command_type = match command_type {
            Some(cmd) => cmd,
            None => Self::Unknown,
        };

        match command_type {
            Command::Init | Command::InitAck | Command::Health | Command::HealthAck => command_type,
            _ => Self::Unknown,
        }
    }
}
