use crate::coords::{Latitude, Longitude, Speed};
use crate::datetime::{Date, DateTime, Time};
use crate::Source;
use core::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct RMC {
    pub source: Source,
    pub datetime: DateTime,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub speed: Speed,
    pub course: f32,
    pub magnetic: Option<f32>,
    pub mode: RMCMode,
}

impl RMC {
    pub(crate) fn parse_rmc<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<RMC>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next().and_then(empty_str_as_none))?;
        let f_status = fields.next().and_then(empty_str_as_none);
        let latitude = Latitude::parse(
            fields.next().and_then(empty_str_as_none),
            fields.next().and_then(empty_str_as_none),
        )?;
        let longitude = Longitude::parse(
            fields.next().and_then(empty_str_as_none),
            fields.next().and_then(empty_str_as_none),
        )?;        
        let speed = Speed::parse_from_knots(fields.next().and_then(empty_str_as_none))?;
        let f_course = fields.next().and_then(empty_str_as_none);
        let date = Date::parse_from_ddmmyy(fields.next().and_then(empty_str_as_none))?;
        let f_magnetic = fields.next().and_then(empty_str_as_none);
        let f_magnetic_ew = fields.next().and_then(empty_str_as_none);
        let f_mode = fields.next().and_then(empty_str_as_none);

        let datetime = DateTime::from_date_and_time(date, time)?;
        let status = f_status.and_then(|s| match s {
            "A" => Some(true),
            "V" => Some(false),
            _ => None,
        });        
        let course = if let Some(crs) = f_course {
            Some(
                crs.parse::<f32>()
                    .map_err(|_| "Wrong course field format")?,
            )
        } else {
            None
        };
        let mode = if let Some(m) = f_mode {
            RMCMode::try_from(m)?
        } else {
            if let Some(true) = status {
                RMCMode::Autonomous
            } else {
                RMCMode::NotValid
            }
        };
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
                magnetic: None,
                mode,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RMCMode {
    Autonomous,
    Differential,
    Estimated,
    Manual,
    Simulator,
    NotValid,
}

impl RMCMode {
    pub fn is_valid(&self) -> bool {
        match self {
            RMCMode::Autonomous => true,
            RMCMode::Differential => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for RMCMode {
    type Error = &'static str;

    fn try_from(from: &str) -> Result<Self, &'static str> {
        match from {
            "A" => Ok(RMCMode::Autonomous),
            "D" => Ok(RMCMode::Differential),
            "E" => Ok(RMCMode::Estimated),
            "M" => Ok(RMCMode::Manual),
            "S" => Ok(RMCMode::Simulator),
            "N" => Ok(RMCMode::NotValid),
            _ => Err("Wrong RMC mode character!"),
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
