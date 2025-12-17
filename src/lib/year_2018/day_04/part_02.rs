use crate::year_2018::day_04::event::{Act, Event};
use chrono::{Duration, Timelike};
use std::collections::HashMap;

pub fn part_02(input: &[Event]) -> u32 {
    let guard_minute_map: HashMap<(u32, u32), u32> = input
        .iter()
        .filter(|event| event.act == Some(Act::Sleep))
        .fold(
            HashMap::new(),
            |mut guard_minute_map: HashMap<(u32, u32), u32>, event: &Event| {
                for i in 0..event.minutes {
                    let current_minute =
                        (event.start.unwrap().time() + Duration::minutes(i as i64)).minute();
                    *guard_minute_map
                        .entry((event.guard_id.unwrap(), current_minute))
                        .or_default() += 1;
                }
                guard_minute_map
            },
        );

    let (sleepiest_guard, sleepiest_minute) = *guard_minute_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .0;
    sleepiest_guard * sleepiest_minute
}
