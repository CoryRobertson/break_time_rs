use chrono::{DateTime, Local};

/// ProgramState represents what the program is doing related to rendering
/// TakingBreak(time) is for when the program should be rendering the break overlay
/// Working(time) is for when the program should not be overlaying anything
///
/// Date time inside represents the time that this break started
pub enum ProgramState {
    TakingBreak(DateTime<Local>),
    Working(DateTime<Local>),
    Paused,
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::Working(Local::now())
    }
}
