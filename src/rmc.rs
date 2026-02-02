use crate::coords::{Course, Latitude, Longitude, MagneticCourse, Speed};
use crate::datetime::{Date, DateTime, Time};
use crate::modes::{Mode, Status};
use crate::Source;

/// Recommended Minimum Sentence for any GNSS source.
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RMC {
    /// Navigational system.
    pub source: Source,
    /// Date and time of fix in UTC.
    pub datetime: DateTime,
    /// Latitude in reference datum, mostly WGS-84.
    pub latitude: Latitude,
    /// Logitude in reference datum, mostly WGS-84.
    pub longitude: Longitude,
    /// Speed over ground.
    pub speed: Speed,
    /// Course over ground. Some receivers do not report it when no movement.
    pub course: Option<Course>,
    /// Magnetic course over ground (angle to magnetic North pole). Receiver may not report it.
    pub magnetic: Option<MagneticCourse>,
    /// Receiver's mode of operation.
    pub mode: Mode,
}

impl RMC {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next())?;
        let status = if let Some(f_status) = fields.next() {
            Status::from_str(f_status)?
        } else {
            return Err("Status field is mandatory for RMC sentence!");
        };
        let latitude = Latitude::parse(fields.next(), fields.next())?;
        let longitude = Longitude::parse(fields.next(), fields.next())?;
        let speed = Speed::parse(fields.next())?;
        let course = Course::parse(fields.next())?;
        let date = Date::parse_from_ddmmyy(fields.next())?;
        let magnetic = MagneticCourse::parse_from_mvar_mdir(&course, fields.next(), fields.next())?;
        let mode = Mode::from_some_str_or_status(fields.next(), &status)?;

        let datetime = DateTime::from_date_and_time(date, time)?;
        if let (Some(datetime), Some(latitude), Some(longitude), Some(speed)) =
            (datetime, latitude, longitude, speed)
        {
            Ok(Some(RMC {
                source,
                datetime,
                latitude,
                longitude,
                speed,
                course,
                magnetic: magnetic,
                mode,
            }))
        } else {
            Ok(None)
        }
    }
}
