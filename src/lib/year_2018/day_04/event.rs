use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub enum Act {
    Begin,
    Sleep,
    Wake,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub guard_id: Option<u32>,
    pub act: Option<Act>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub minutes: u32,
}

impl Event {
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
