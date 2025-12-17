use chrono::NaiveDateTime;

use crate::year_2018::day_04::{act::Act, event::Event};

pub fn get_input() -> Vec<Event> {
    let mut eventlog: Vec<Event> = include_str!("input.txt")
        .lines()
        .map(|line| {
            // guard_id
            let mut guard_id: Option<u32> = None;
            if let (Some(i), Some(j)) = (line.find("Guard #"), line.find(" begins shift")) {
                guard_id = Some(line[i + 7..j].parse::<u32>().expect(
                    format!("cannot convert {} to u32", line[i + 7..j].to_string()).as_str(),
                ));
            }

            // act
            let act = if line.contains(&"begin") {
                Some(Act::Begin)
            } else if line.contains(&"sleep") {
                Some(Act::Sleep)
            } else if line.contains(&"wake") {
                Some(Act::Wake)
            } else {
                None
            };

            // start
            let start = Some(
                NaiveDateTime::parse_from_str(
                    &line[line.find("[").unwrap() + 1..line.find("]").unwrap()],
                    "%Y-%m-%d %H:%M",
                )
                .expect("cannot convert to NaiveDateTime"),
            );

            // end
            let end = None;

            // minutes
            let minutes = 0;

            Event::new(guard_id, act, start, end, minutes)
        })
        .collect();

    // sort by when
    eventlog.sort_by(|x, y| x.start.cmp(&y.start));

    // further adjusting
    let mut cached_guard_id: Option<u32> = None;
    for i in 0..eventlog.len() {
        // guard_id
        if let Some(guard_id) = eventlog[i].guard_id {
            cached_guard_id = Some(guard_id);
        } else {
            eventlog[i].guard_id = cached_guard_id;
        }

        if i > 0 {
            // end
            eventlog[i - 1].end = Some(eventlog[i].start.unwrap());

            // minutes
            eventlog[i - 1].minutes = (eventlog[i - 1].end.unwrap()
                - eventlog[i - 1].start.unwrap())
            .num_minutes() as u32;
        }
    }
    eventlog
}
