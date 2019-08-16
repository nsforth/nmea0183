use crate::coords::{Latitude, Longitude};
use crate::datetime;
use crate::Source;
use core::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct RMC {
    pub source: Source,
    pub datetime: datetime::DateTime,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub speed: f32,
    pub course: f32,
    pub magnetic: Option<f32>,
    pub mode: RMCMode,
}

impl RMC {
    pub(crate) fn parse_rmc<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<RMC>, &'static str> {
        // Extracting fields as borrowed strings. None if not present or empty
        let f_time = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_status = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_lat = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_lat_ns = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_lon = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_lon_ew = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_speed = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_course = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_date = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_magnetic = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_magnetic_ew = fields.next().and_then(RMC::get_empty_str_as_none);
        let f_mode = fields.next().and_then(RMC::get_empty_str_as_none);

        let datetime = if let (Some(t), Some(d)) = (f_time, f_date) {
            Some(datetime::DateTime::parse_from_ddmmyy_hhmmss(d, t)?)
        } else {
            None
        };
        let status = f_status.and_then(|s| match s {
            "A" => Some(true),
            "V" => Some(false),
            _ => None,
        });
        let latitude = Latitude::parse(f_lat, f_lat_ns)?;
        let longitude = Longitude::parse(f_lon, f_lon_ew)?;
        let speed = if let Some(spd) = f_speed {
            Some(spd.parse::<f32>().map_err(|_| "Wrong speed field format")?)
        } else {
            None
        };
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

    fn get_empty_str_as_none(s: &str) -> Option<&str> {
        if s.len() == 0 {
            None
        } else {
            Some(s)
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
