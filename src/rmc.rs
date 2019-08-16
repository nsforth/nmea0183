use crate::coords::{Course, Latitude, Longitude, MagneticCourse, Speed};
use crate::datetime::{Date, DateTime, Time};
use crate::modes::{Mode, Status};
use crate::Source;

#[derive(Debug, PartialEq)]
pub struct RMC {
    pub source: Source,
    pub datetime: DateTime,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub speed: Speed,
    pub course: Course,
    pub magnetic: Option<MagneticCourse>,
    pub mode: Mode,
}

impl RMC {
    pub(crate) fn parse_rmc<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<RMC>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next().and_then(empty_str_as_none))?;
        let status = if let Some(f_status) = fields.next() {
            Status::from_str(f_status)?
        } else {
            return Err("Status field is mandatory for RMC sentence!");
        };
        let latitude = Latitude::parse(
            fields.next().and_then(empty_str_as_none),
            fields.next().and_then(empty_str_as_none),
        )?;
        let longitude = Longitude::parse(
            fields.next().and_then(empty_str_as_none),
            fields.next().and_then(empty_str_as_none),
        )?;
        let speed = Speed::parse(fields.next().and_then(empty_str_as_none))?;
        let course = Course::parse(fields.next().and_then(empty_str_as_none))?;
        let date = Date::parse_from_ddmmyy(fields.next().and_then(empty_str_as_none))?;
        let magnetic = MagneticCourse::parse(
            &course,
            fields.next().and_then(empty_str_as_none),
            fields.next().and_then(empty_str_as_none),
        )?;
        let mode = Mode::from_some_str_or_status(fields.next(), &status)?;

        let datetime = DateTime::from_date_and_time(date, time)?;
        if let (Some(dt), Some(lat), Some(lon), Some(spd), Some(crs)) =
            (datetime, latitude, longitude, speed, course)
        {
            Ok(Some(RMC {
                source,
                datetime: dt,
                latitude: lat,
                longitude: lon,
                speed: spd,
                course: crs,
                magnetic: magnetic,
                mode,
            }))
        } else {
            Ok(None)
        }
    }
}

fn empty_str_as_none(s: &str) -> Option<&str> {
    if s.len() == 0 {
        None
    } else {
        Some(s)
    }
}
