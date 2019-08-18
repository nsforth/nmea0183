#[derive(Debug, PartialEq)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub(crate) fn parse_from_ddmmyy(input: Option<&str>) -> Result<Option<Date>, &'static str> {
        match input {
            Some(date) if date.len() == 0 => Ok(None),
            Some(date) if date.len() < 6 => Err("Date input string is too short!"),
            Some(date) => Ok(Some(Date {
                day: (&date[..2])
                    .parse()
                    .map_err(|_| "Day string is not a number!")?,
                month: (&date[2..4])
                    .parse()
                    .map_err(|_| "Month string is not a number!")
                    .and_then(|m| {
                        if m > 0 && m < 13 {
                            Ok(m)
                        } else {
                            Err("Months is not in range 1-12")
                        }
                    })?,
                year: (&date[4..6])
                    .parse::<u16>()
                    .map(|year| if year > 69 { year + 1900 } else { year + 2000 })
                    .map_err(|_| "Year string is not a number!")?,
            })),
            _ => Ok(None),
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
    pub(crate) fn parse_from_hhmmss(input: Option<&str>) -> Result<Option<Time>, &'static str> {
        match input {
            Some(time) if time.len() == 0 => Ok(None),
            Some(time) if time.len() < 6 => Err("Date input string is too short!"),
            Some(time) => Ok(Some(Time {
                hours: (&time[..2])
                    .parse()
                    .map_err(|_| "Hours string is not a number!")
                    .and_then(|h| {
                        if h < 24 {
                            Ok(h)
                        } else {
                            Err("Hours is not in range 0-23")
                        }
                    })?,
                minutes: (&time[2..4])
                    .parse()
                    .map_err(|_| "Minutes string is not a number!")
                    .and_then(|m| {
                        if m < 59 {
                            Ok(m)
                        } else {
                            Err("Minutes is not in range 0-59")
                        }
                    })?,
                seconds: (&time[4..])
                    .parse::<f32>()
                    .map_err(|_| "Seconds string is not a float")
                    .and_then(|s| {
                        if s < 60f32 {
                            Ok(s)
                        } else {
                            Err("Seconds is not in range 0-59")
                        }
                    })?,
            })),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub(crate) fn from_date_and_time(
        maybe_date: Option<Date>,
        maybe_time: Option<Time>,
    ) -> Result<Option<Self>, &'static str> {
        match (maybe_date, maybe_time) {
            (Some(date), Some(time)) => Ok(Some(DateTime { date, time })),
            (None, None) => Ok(None),
            _ => Err("Date or time is None, should be Some both"),
        }
    }
}

#[test]
fn test_parse_date() {
    let date = Date::parse_from_ddmmyy(Some("010210")).unwrap().unwrap();
    assert_eq!(date.day, 1);
    assert_eq!(date.month, 2);
    assert_eq!(date.year, 2010);
    let date = Date::parse_from_ddmmyy(Some("010270")).unwrap().unwrap();
    assert_eq!(date.day, 1);
    assert_eq!(date.month, 2);
    assert_eq!(date.year, 1970);
}

#[test]
fn test_parse_time() {
    let time = Time::parse_from_hhmmss(Some("124201.340"))
        .unwrap()
        .unwrap();
    assert_eq!(time.hours, 12);
    assert_eq!(time.minutes, 42);
    assert_eq!(time.seconds, 1.34);
}
