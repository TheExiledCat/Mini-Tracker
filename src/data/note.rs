use crate::data::effect::Effect;

pub const MAX_TRACK_EFFECTS: usize = 4;
/// Type safe wrapper for a handle on an instrument
#[derive(Debug, Clone)]
pub struct InstrumentId(pub u8);
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: u8,
    pub instrument_id: InstrumentId,
    pub volume: u8,
    pub effects: [Option<Effect>; MAX_TRACK_EFFECTS],
}
#[derive(Debug, Clone)]
pub enum NoteEvent {
    Empty,
    Note(Note),
}
