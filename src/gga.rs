use crate::common;
use crate::coords::{Altitude, Latitude, Longitude};
use crate::datetime::Time;
use crate::Source;
use core::time::Duration;

/// Geographic coordinates including altitude, GPS solution quality, DGPS usage information.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GGA {
    /// Navigational system.
    pub source: Source,
    /// Time of fix in UTC.
    pub time: Time,
    /// Latitude in reference datum, typically WGS-84.
    pub latitude: Latitude,
    /// Logitude in reference datum, typically WGS-84.
    pub longitude: Longitude,
    /// Quality of GPS solution.
    pub gps_quality: GPSQuality,
    /// Sattelites in use
    pub sat_in_use: u8,
    /// Horizontal dilusion of presicion. Indicates precision of solution.
    pub hdop: f32,
    /// Altitude over ground, typically WGS-84.
    pub altitude: Option<Altitude>,
    /// The difference between reference ellipsoid surface and mean-sea-level.
    pub geoidal_separation: Option<f32>,
    /// DGPS data age. None if DGPS not in use.
    pub age_dgps: Option<Duration>,
    /// ID of reference DGPS station used for fix. None if DGPS not in use.
    pub dgps_station_id: Option<u16>,
}

impl GGA {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next())?;
        let latitude = Latitude::parse(fields.next(), fields.next())?;
        let longitude = Longitude::parse(fields.next(), fields.next())?;
        let gps_quality = GPSQuality::parse(fields.next())?;
        let sat_in_use = common::parse_u8(fields.next())?;
        let hdop = common::parse_f32(fields.next())?;
        let altitude = Altitude::parse(fields.next())?;
        fields.next(); // Skip altitude type (always meters according to NMEA spec)
        let geoidal_separation = common::parse_f32(fields.next())?;
        fields.next(); // Skip geoidal separation type (always meters according to NMEA spec)
        let age_dgps = common::parse_f32(fields.next())?
            .and_then(|a| Some(Duration::from_millis((a * 1000f32) as u64)));
        let dgps_station_id = common::parse_u16(fields.next())?;
        if let (
            Some(time),
            Some(latitude),
            Some(longitude),
            Some(gps_quality),
            Some(sat_in_use),
            Some(hdop),
        ) = (time, latitude, longitude, gps_quality, sat_in_use, hdop)
        {
            Ok(Some(GGA {
                source,
                time,
                latitude,
                longitude,
                gps_quality,
                sat_in_use,
                hdop,
                altitude,
                geoidal_separation,
                age_dgps,
                dgps_station_id,
            }))
        } else {
            Ok(None)
        }
    }
}

/// Quality of GPS solution
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPSQuality {
    /// No solution
    NoFix,
    /// Ordinary GPS solution
    GPS,
    /// Differential correction used.
    DGPS,
    /// Locked PPS (pulse per second).
    PPS,
    /// RTK correction is in use.
    RTK,
    /// Float RTK correction is in use.
    FRTK,
    /// Estimated by movement model.
    Estimated,
    /// Set by operator.
    Manual,
    /// Simulated.
    Simulated,
}

impl GPSQuality {
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<GPSQuality>, &'static str> {
        match input {
            Some("0") => Ok(Some(GPSQuality::NoFix)),
            Some("1") => Ok(Some(GPSQuality::GPS)),
            Some("2") => Ok(Some(GPSQuality::DGPS)),
            Some("3") => Ok(Some(GPSQuality::PPS)),
            Some("4") => Ok(Some(GPSQuality::RTK)),
            Some("5") => Ok(Some(GPSQuality::FRTK)),
            Some("6") => Ok(Some(GPSQuality::Estimated)),
            Some("7") => Ok(Some(GPSQuality::Manual)),
            Some("8") => Ok(Some(GPSQuality::Simulated)),
            Some("") => Ok(None),
            None => Ok(None),
            _ => Err("Wrong GPSQuality indicator type!"),
        }
    }
}

#[test]
fn test_parse_gpsquality() {
    assert_eq!(GPSQuality::parse(Some("0")), Ok(Some(GPSQuality::NoFix)));
    assert_eq!(GPSQuality::parse(Some("1")), Ok(Some(GPSQuality::GPS)));
    assert_eq!(GPSQuality::parse(Some("2")), Ok(Some(GPSQuality::DGPS)));
    assert_eq!(GPSQuality::parse(Some("3")), Ok(Some(GPSQuality::PPS)));
    assert_eq!(GPSQuality::parse(Some("4")), Ok(Some(GPSQuality::RTK)));
    assert_eq!(GPSQuality::parse(Some("5")), Ok(Some(GPSQuality::FRTK)));
    assert_eq!(
        GPSQuality::parse(Some("6")),
        Ok(Some(GPSQuality::Estimated))
    );
    assert_eq!(GPSQuality::parse(Some("7")), Ok(Some(GPSQuality::Manual)));
    assert_eq!(
        GPSQuality::parse(Some("8")),
        Ok(Some(GPSQuality::Simulated))
    );
    assert_eq!(GPSQuality::parse(Some("")), Ok(None));
    assert_eq!(GPSQuality::parse(None), Ok(None));
    assert!(GPSQuality::parse(Some("9")).is_err());
}
