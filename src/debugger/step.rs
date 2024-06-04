use crate::debugger::parser_data::ParserData;

/**************************************** Struct and Type definitions ***************************************************/

pub struct StepData {
    pub addresses: ParserData,
    pub is_stepping: bool,
    pub steps_to_run: usize,
}

impl StepData {
    pub fn new() -> Self {
        Self {
            addresses: ParserData::new(),
            is_stepping: false,
            steps_to_run: 0,
        }
    }
}
