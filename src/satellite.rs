//! Structures that describe satellites in views .

use crate::common;
use crate::Source;

///Information about satellite in view.
#[derive(Debug, PartialEq, Clone)]
pub struct Satellite {
    /// Satellite pseudo-random noise number.
    pub prn: Prn,
    /// Angle of elevation of the satellite above the horizontal plane in degrees, 90° maximum.
    pub elevation: u8,
    /// Degrees from True North, 000° through 359°
    pub azimuth: u16,
    /// Signal-to-Noise Ratio . 00 through 99 dB (null when not tracking) .
    pub snr: Option<u8>,
}

impl Satellite {
    pub(crate) fn parse<'a>(
        fields: &mut core::str::Split<'a, char>,
        source: Source,
    ) -> Result<Option<Self>, &'static str> {
        let prn = Prn::parse(fields.next(), source)?;
        let elevation = common::parse_u8(fields.next())?;
        let azimuth = common::parse_u16(fields.next())?;
        let snr = common::parse_u8(fields.next())?;

        if let (Some(prn), Some(elevation), Some(azimuth)) = (prn, elevation, azimuth) {
            Ok(Some(Self {
                prn,
                elevation,
                azimuth,
                snr,
            }))
        } else {
            Ok(None)
        }
    }
}

///Satellite pseudo-random noise number
#[derive(Debug, PartialEq, Clone)]
pub struct Prn {
    /// Satellite pseudo-random noise number based on source of NMEA sentence
    pub number: u8,
}

impl Prn {
    pub(crate) fn parse(input: Option<&str>, source: Source) -> Result<Option<Self>, &'static str> {
        if let Some(input) = input {
            if input.len() == 0 {
                Ok(None)
            } else {
                let original_prn_number =
                    input.parse::<u8>().map_err(|_| "Wrong prn field format")?;

                let adjusted_prn_number = match source {
                    Source::Beidou => original_prn_number.checked_sub(100),
                    Source::GLONASS => original_prn_number.checked_sub(64),
                    _ => Some(original_prn_number),
                };
                match adjusted_prn_number {
                    Some(number) => Ok(Some(Prn { number })),
                    None => Err("Could not parse Prn: number not match GNSS system Prn range"),
                }
            }
        } else {
            Ok(None)
        }
    }
}
