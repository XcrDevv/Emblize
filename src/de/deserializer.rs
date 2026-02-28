use crate::core::reader::Reader;

#[derive(Debug, PartialEq, Eq)]
pub enum DeState {
    ReadingValue,
    ReadingField,
    ReadingSeq,
    ReadingFixedSeq(usize),
    ReadingTime,
}

pub struct Deserializer<'de> {
    pub input: Reader<'de>,
    pub state: DeState,
}

impl<'de> Deserializer<'de> {
        pub fn new(input: Reader<'de>) -> Self {
        Self {
            input,

            state: DeState::ReadingValue,
        }
    }
}