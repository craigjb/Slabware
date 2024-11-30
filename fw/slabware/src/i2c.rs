#[derive(Clone, Copy, PartialEq)]
pub enum AckKind {
    Ack,
    Nack,
}

impl AckKind {
    pub fn value(&self) -> bool {
        match self {
            Self::Ack => false,
            Self::Nack => true,
        }
    }

    pub fn from_value(value: bool) -> Self {
        match value {
            false => Self::Ack,
            true => Self::Nack,
        }
    }

    pub fn is_nack(&self) -> bool {
        *self == AckKind::Nack
    }
}
