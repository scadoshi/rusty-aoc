use crate::year_2018::day_04::act::Act;
use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Event {
    pub guard_id: Option<u32>,
    pub act: Option<Act>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub minutes: u32,
}

impl Event {
    #[allow(dead_code)]
    pub fn new(
        guard_id: Option<u32>,
        act: Option<Act>,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
        minutes: u32,
    ) -> Self {
        Self {
            guard_id,
            act,
            start,
            end,
            minutes,
        }
    }
}
