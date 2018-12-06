extern crate chrono;
extern crate regex;

use std::str::FromStr;
use regex::Regex;
use regex::Captures;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;

pub type GuardSleepMinutes = HashMap<u32, u32>;
pub type GuardSleepInfo = HashMap<u32, GuardSleepMinutes>;

#[derive(Debug, PartialEq)]
pub enum GuardEvent {
    StartOfShift { datetime: NaiveDateTime, id: u32 },
    Sleep { datetime: NaiveDateTime },
    WakeUp { datetime: NaiveDateTime },
}

pub fn get_most_sleeping_guard(sleep_info: &GuardSleepInfo) -> (u32, u32, u32) {
    let most_sleepy_guard: (u32, u32, u32) = sleep_info
        .iter()
        .fold((0, 0, 0), |previous, (guard, sleep)| {
            let total_sleep = sleep
                .iter()
                .fold(0, |total, (_minute, amount)| total + amount);

            let most_sleepy_minute: (u32, u32) = sleep
                .iter()
                .fold((0, 0), |prev, next| {
                    if prev.1 > *next.1 {
                        prev
                    } else {
                        (*next.0, *next.1)
                    }
                });

            if previous.1 > total_sleep {
                previous
            } else {
                (guard.clone(), total_sleep, most_sleepy_minute.0)
            }
        });

    most_sleepy_guard
}

pub fn get_most_sleeped_minute(sleep_info: &GuardSleepInfo) -> (u32, u32, u32) {
    let most_sleepy_guard: (u32, u32, u32) = sleep_info
        .iter()
        .fold((0, 0, 0), |previous, (guard, sleep)| {

            let most_sleepy_minute: (u32, u32) = sleep
                .iter()
                .fold((0, 0), |prev, next| {
                    if prev.1 > *next.1 {
                        prev
                    } else {
                        (*next.0, *next.1)
                    }
                });

            if previous.2 > most_sleepy_minute.1 {
                previous
            } else {
                (guard.clone(), most_sleepy_minute.0, most_sleepy_minute.1)
            }
        });

    most_sleepy_guard
}

pub fn events_to_sleep_info(events: &Vec<GuardEvent>) -> GuardSleepInfo {
    let mut current_guard: u32 = 0;
    let mut fall_asleep_at: u32 = 0;

    events.iter().fold(
        HashMap::new(),
        |mut all, event| {
            {
                let minutes_asleep = all
                    .entry(current_guard)
                    .or_insert(
                        HashMap::new()
                    );

                match event {
                    GuardEvent::StartOfShift { datetime, id } => {
                        current_guard = id.clone();
                    }
                    GuardEvent::Sleep { datetime } => {
                        fall_asleep_at = datetime.minute();
                    }
                    GuardEvent::WakeUp { datetime } => {
                        let awake = datetime.minute();
                        for m in fall_asleep_at..awake {
                            let mut minute = minutes_asleep
                                .entry(m)
                                .or_insert(0);

                            *minute += 1;
                        }
                    }
                }
            }
            all
        })
}

pub fn sort_events(events: &mut Vec<GuardEvent>) {
    events.sort_by(|a, b| {
        let date1 = match a {
            GuardEvent::StartOfShift { datetime, .. } => datetime,
            GuardEvent::Sleep { datetime } => datetime,
            GuardEvent::WakeUp { datetime } => datetime
        };

        let date2 = match b {
            GuardEvent::StartOfShift { datetime, .. } => datetime,
            GuardEvent::Sleep { datetime } => datetime,
            GuardEvent::WakeUp { datetime } => datetime
        };

        date1.cmp(&date2)
    });
}

impl FromStr for GuardEvent {
    type Err = ();
    fn from_str(input: &str) -> Result<GuardEvent, ()> {
        let start_of_shift =
            Regex::new(r"\[(?P<date>.*)\] Guard \#(?P<id>\d+) .*$").unwrap();
        let sleep =
            Regex::new(r"\[(?P<date>.*)\] falls asleep$").unwrap();
        let wake =
            Regex::new(r"\[(?P<date>.*)\] wakes up").unwrap();

        match start_of_shift.captures(input) {
            Some(caps) => {
                let datetime = parse_date(&caps);
                let id: u32 = caps["id"].parse().unwrap();
                Ok(GuardEvent::StartOfShift { datetime, id })
            }
            None => {
                match sleep.captures(input) {
                    Some(caps) => {
                        let datetime = parse_date(&caps);
                        Ok(GuardEvent::Sleep { datetime })
                    }
                    None => {
                        match wake.captures(input) {
                            Some(caps) => {
                                let datetime = parse_date(&caps);
                                Ok(GuardEvent::WakeUp { datetime })
                            }
                            None => Err(())
                        }
                    }
                }
            }
        }
    }
}

fn parse_date(caps: &Captures) -> NaiveDateTime {
    let input = &caps["date"];
    NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M").unwrap()
}

#[cfg(test)]
mod tests {
    use GuardEvent;
    use chrono::NaiveDate;
    use sort_events;
    use events_to_sleep_info;

    #[test]
    fn it_should_sort_input() {
        // Arrange
        let event1: GuardEvent = "[1518-11-01 00:00] Guard #1 begins shift".parse().unwrap();
        let event2: GuardEvent = "[1518-11-01 00:03] Guard #4 begins shift".parse().unwrap();
        let event3: GuardEvent = "[1518-11-01 00:02] Guard #3 begins shift".parse().unwrap();
        let event4: GuardEvent = "[1518-11-01 00:01] Guard #2 begins shift".parse().unwrap();
        let mut events = vec![&event1, &event2, &event3, &event4];

        // Act
        sort_events(&mut events);

        // Assert
        assert_eq!(events[0], &event1);
        assert_eq!(events[1], &event4);
        assert_eq!(events[2], &event3);
        assert_eq!(events[3], &event2);
    }

    #[test]
    fn it_should_parse_start_of_shifts() {
        // Arrange
        let datetime = NaiveDate::from_ymd(1518, 11, 1)
            .and_hms(0, 0, 0);

        // Act
        let event: GuardEvent = "[1518-11-01 00:00] Guard #10 begins shift".parse().unwrap();

        // Assert
        assert_eq!(event, GuardEvent::StartOfShift { datetime, id: 10 });
    }

    #[test]
    fn it_should_parse_sleep() {
        // Arrange
        let datetime = NaiveDate::from_ymd(1518, 11, 1)
            .and_hms(0, 5, 0);

        // Act
        let event: GuardEvent = "[1518-11-01 00:05] falls asleep".parse().unwrap();

        // Assert
        assert_eq!(event, GuardEvent::Sleep { datetime });
    }

    #[test]
    fn it_should_parse_wake_up() {
        // Arrange
        let datetime = NaiveDate::from_ymd(1518, 11, 1)
            .and_hms(0, 25, 0);

        // Act
        let event: GuardEvent = "[1518-11-01 00:25] wakes up".parse().unwrap();

        // Assert
        assert_eq!(event, GuardEvent::WakeUp { datetime });
    }
}
