use crate::coords::{Altitude, Latitude, Longitude};
use crate::datetime::Time;
use crate::Source;
use core::time::Duration;

#[derive(Debug, PartialEq)]
pub struct GGA {
    pub source: Source,
    pub time: Time,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub gps_quality: GPSQuality,
    pub sat_in_use: u8,
    pub hdop: f32,
    pub altitude: Altitude,
    pub geoidal_separation: Option<f32>,
    pub age_dgps: Option<Duration>,
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
        let sat_in_use = if let Some(f_sat_in_use) = fields.next() {
            if f_sat_in_use.len() == 0 {
                None
            } else {
                Some(
                    f_sat_in_use
                        .parse::<u8>()
                        .map_err(|_| "Wrong sattelites in use field format")?,
                )
            }
        } else {
            None
        };
        let hdop = if let Some(f_hdop) = fields.next() {
            if f_hdop.len() == 0 {
                None
            } else {
                Some(
                    f_hdop
                        .parse::<f32>()
                        .map_err(|_| "Wrong horizontal DOP field format")?,
                )
            }
        } else {
            None
        };
        let altitude = Altitude::parse(fields.next())?;
        fields.next(); // Skip altitude type (always meters according to NMEA spec)
        let geoidal_separation = if let Some(f_geoidal_separation) = fields.next() {
            if f_geoidal_separation.len() == 0 {
                None
            } else {
                Some(
                    f_geoidal_separation
                        .parse::<f32>()
                        .map_err(|_| "Wrong geoidal separation field format")?,
                )
            }
        } else {
            None
        };
        fields.next(); // Skip geoidal separation type (always meters according to NMEA spec)
        let age_dgps = if let Some(f_age_dgps) = fields.next() {
            if f_age_dgps.len() == 0 {
                None
            } else {
                Some(
                    f_age_dgps
                        .parse::<f32>()
                        .map_err(|_| "Wrong age dgps field format")
                        .and_then(|a| Ok(Duration::from_millis((a * 1000f32) as u64)))?,
                )
            }
        } else {
            None
        };
        let dgps_station_id = if let Some(f_dgps_station_id) = fields.next() {
            if f_dgps_station_id.len() == 0 {
                None
            } else {
                Some(
                    f_dgps_station_id
                        .parse::<u16>()
                        .map_err(|_| "Wrong diff station id field format")?,
                )
            }
        } else {
            None
        };
        if let (
            Some(time),
            Some(latitude),
            Some(longitude),
            Some(gps_quality),
            Some(sat_in_use),
            Some(hdop),
            Some(altitude),
        ) = (
            time,
            latitude,
            longitude,
            gps_quality,
            sat_in_use,
            hdop,
            altitude,
        ) {
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

#[derive(Debug, PartialEq)]
pub enum GPSQuality {
    NoFix,
    GPS,
    DGPS,
    PPS,
    RTK,
    FRTK,
    Estimated,
    Manual,
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
