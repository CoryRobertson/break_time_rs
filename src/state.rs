use chrono::{DateTime, Local};

/// ProgramState represents what the program is doing related to rendering
/// TakingBreak(time) is for when the program should be rendering the break overlay
/// Working(time) is for when the program should not be overlaying anything, if the option is none, that means it is work time, but the user has not generated activity to being working
///
/// Date time inside represents the time that this break started
#[derive(Debug)]
pub enum ProgramState {
    TakingBreak(DateTime<Local>),
    Working(Option<DateTime<Local>>),
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::Working(Option::from(Local::now()))
    }
}
