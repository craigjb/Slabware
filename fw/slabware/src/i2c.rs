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
}
