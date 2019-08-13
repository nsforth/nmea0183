#[derive(Debug, PartialEq)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    fn parse_from_ddmmyy(input: &str) -> Result<Date, &'static str> {
        if input.len() < 6 {
            Err("Date input string is too short!")
        } else {
            Ok(Date {
                day: (&input[..2])
                    .parse()
                    .map_err(|_| "Day string is not a number!")?,
                month: (&input[2..4])
                    .parse()
                    .map_err(|_| "Month string is not a number!")
                    .and_then(|m| {
                        if m > 0 && m < 13 {
                            Ok(m)
                        } else {
                            Err("Months is not in range 1-12")
                        }
                    })?,
                year: (&input[4..6])
                    .parse::<u16>()
                    .map(|year| if year > 69 { year + 1900 } else { year + 2000 })
                    .map_err(|_| "Year string is not a number!")?,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: f32,
}

impl Time {
    fn parse_from_hhmmss(input: &str) -> Result<Time, &'static str> {
        if input.len() < 6 {
            Err("Date input string is too short!")
        } else {
            Ok(Time {
                hours: (&input[..2])
                    .parse()
                    .map_err(|_| "Hours string is not a number!")
                    .and_then(|h| {
                        if h < 24 {
                            Ok(h)
                        } else {
                            Err("Hours is not in range 0-23")
                        }
                    })?,
                minutes: (&input[2..4])
                    .parse()
                    .map_err(|_| "Minutes string is not a number!")
                    .and_then(|m| {
                        if m < 59 {
                            Ok(m)
                        } else {
                            Err("Minutes is not in range 0-59")
                        }
                    })?,
                seconds: (&input[4..])
                    .parse::<f32>()
                    .map_err(|_| "Seconds string is not a float")
                    .and_then(|s| {
                        if s < 60f32 {
                            Ok(s)
                        } else {
                            Err("Seconds is not in range 0-59")
                        }
                    })?,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub(crate) fn parse_from_ddmmyy_hhmmss(
        input_date: &str,
        input_time: &str,
    ) -> Result<DateTime, &'static str> {
        let date = Date::parse_from_ddmmyy(input_date)?;
        let time = Time::parse_from_hhmmss(input_time)?;
        Ok(DateTime { date, time })
    }
}

#[test]
fn test_parse_date() {
    let date = Date::parse_from_ddmmyy("010210").unwrap();
    assert_eq!(date.day, 1);
    assert_eq!(date.month, 2);
    assert_eq!(date.year, 2010);
    let date = Date::parse_from_ddmmyy("010270").unwrap();
    assert_eq!(date.day, 1);
    assert_eq!(date.month, 2);
    assert_eq!(date.year, 1970);
}

#[test]
fn test_parse_time() {
    let time = Time::parse_from_hhmmss("124201.340").unwrap();
    assert_eq!(time.hours, 12);
    assert_eq!(time.minutes, 42);
    assert_eq!(time.seconds, 1.34);
}
