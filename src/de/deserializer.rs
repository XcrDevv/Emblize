use crate::core::reader::Reader;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeState {
    ReadUntypedValue,
    ReadTypedValue,
    ReadingSeq(Option<u8>),
}

pub struct Deserializer<'de> {
    pub input: Reader<'de>,
    pub state: DeState,
}

impl<'de> Deserializer<'de> {
        pub fn new(input: Reader<'de>) -> Self {
        Self {
            input,

            state: DeState::ReadUntypedValue,
        }
    }
}