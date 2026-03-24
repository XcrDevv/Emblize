#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimestampMillis(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimestampMicros(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MillisSinceBoot(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MicrosSinceBoot(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DurationMillis(pub i64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DurationMicros(pub i64);