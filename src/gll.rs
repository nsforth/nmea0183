use crate::coords::{Latitude, Longitude};
use crate::datetime::Time;
use crate::modes::{Mode, Status};
use crate::Source;

/// Geographic latitude ang longitude sentence with time of fix and receiver state.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GLL {
    /// Navigational system.
    pub source: Source,
    /// Time of fix in UTC.
    pub time: Time,
    /// Latitude in reference datum, mostly WGS-84.
    pub latitude: Latitude,
    /// Logitude in reference datum, mostly WGS-84.
    pub longitude: Longitude,
    /// Receiver's mode of operation.
    pub mode: Mode,
}

impl GLL {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let latitude = Latitude::parse(fields.next(), fields.next())?;
        let longitude = Longitude::parse(fields.next(), fields.next())?;
        let time = Time::parse_from_hhmmss(fields.next())?;
        let status = if let Some(f_status) = fields.next() {
            Status::from_str(f_status)?
        } else {
            return Err("Status field is mandatory for GLL sentence!");
        };
        let mode = Mode::from_some_str_or_status(fields.next(), &status)?;
        if let (Some(lat), Some(lon), Some(time)) = (latitude, longitude, time) {
            Ok(Some(GLL {
                source,
                time: time,
                latitude: lat,
                longitude: lon,
                mode,
            }))
        } else {
            Ok(None)
        }
    }
}
