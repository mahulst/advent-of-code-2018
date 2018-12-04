extern crate chrono;
extern crate regex;

use chrono::DateTime;
use chrono::Utc;
use std::str::FromStr;
use regex::Regex;
use regex::Captures;
use chrono::NaiveDateTime;

enum GuardEvent {
    StartOfShift { datetime: NaiveDateTime, id: i32 },
    Sleep { datetime: NaiveDateTime },
    WakeUp { datetime: NaiveDateTime },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

impl FromStr for GuardEvent {
    type Err = ();
    fn from_str(s: &str) -> Result<GuardEvent, ()> {
        let start_of_shift =
            Regex::new(r"\[(?P<date>.*)\] Guard \#(?P<id>\d+) .*$").unwrap();
        let sleep =
            Regex::new(r"\[(?P<date>.*)\] falls asleep$").unwrap();
        let wake =
            Regex::new(r"\[(?P<date>.*)\] wakes up").unwrap();

        let input = "[1518-11-01 00:00] Guard #10 begins shift";


        match start_of_shift.captures(input) {
            Some(caps) => {
                let datetime = parse_date(&caps);
                let id: i32 = caps["id"].parse().unwrap();
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