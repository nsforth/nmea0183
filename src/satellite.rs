//! Structures that describe satellites in views .

use crate::common;

///Information about satellite in view.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Satellite {
    /// Satellite pseudo-random noise number.
    pub prn: u16,
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
    ) -> Result<Option<Self>, &'static str> {
        let prn = common::parse_u16(fields.next())?;
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
