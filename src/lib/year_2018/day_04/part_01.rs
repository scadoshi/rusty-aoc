use crate::year_2018::day_04::event::{Act, Event};
use chrono::Timelike;
use std::collections::HashMap;

pub fn part_01(input: &[Event]) -> u32 {
    // count guard sleep minutes
    let sleep_minutes: HashMap<u32, u32> =
        input
            .iter()
            .fold(HashMap::new(), |mut sleep_log: HashMap<u32, u32>, event| {
                if *event.act.as_ref().unwrap() == Act::Sleep {
                    *sleep_log.entry(event.guard_id.unwrap()).or_default() += event.minutes
                }
                sleep_log
            });
    let sleepiest_guard = *sleep_minutes.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
    let minute_map: HashMap<u32, u32> = input
        .iter()
        .filter(|event| event.guard_id == Some(sleepiest_guard) && event.act == Some(Act::Sleep))
        .fold(
            HashMap::new(),
            |mut minute_map: HashMap<u32, u32>, event| {
                for i in 0..event.minutes {
                    let current_minute = (event.start.unwrap().time()
                        + std::time::Duration::from_mins(i as u64))
                    .minute();
                    *minute_map.entry(current_minute).or_default() += 1;
                }
                minute_map
            },
        );
    let sleepiest_minute = *minute_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
    sleepiest_guard * sleepiest_minute
}
