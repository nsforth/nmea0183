use crate::common;
use crate::modes::Mode;
use crate::Source;
const MAX_PRNS_PER_MESSAGE: usize = 12;

/// GPS DOP and active satellites
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GSA {
    /// Navigational system.
    pub source: Source,
    /// Receiver's mode of operation.
    pub mode: Mode,
    /// Fix type indicating if it's a 2D or 3D fix.
    pub fix_type: FixType,
    /// Array of PRNs for satellites used in the fix.
    fix_sats_prn: [u16; MAX_PRNS_PER_MESSAGE],
    /// The actual number of PRNs in the array.
    prn_array_size: usize,
    /// Position dilusion of precision.
    pub pdop: f32,
    /// Horizontal dilusion of precision.
    pub hdop: f32,
    /// Vertical dilusion of precision.
    pub vdop: f32,
}

impl GSA {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let mode = Mode::from_some_str(fields.next())?;
        let fix_type = FixType::parse(fields.next())?;
        let mut fix_sats_prn: [u16; MAX_PRNS_PER_MESSAGE] = Default::default();
        let mut prn_array_size = 0;
        for prn in fix_sats_prn.iter_mut() {
            if let Some(parsed_prn) = common::parse_u16(fields.next())? {
                *prn = parsed_prn;
                prn_array_size += 1;
            }
        }
        let pdop = common::parse_f32(fields.next())?;
        let hdop = common::parse_f32(fields.next())?;
        let vdop = common::parse_f32(fields.next())?;

        if let (Some(fix_type), Some(pdop), Some(hdop), Some(vdop)) = (fix_type, pdop, hdop, vdop) {
            Ok(Some(GSA {
                source,
                mode,
                fix_type,
                fix_sats_prn,
                prn_array_size,
                pdop,
                hdop,
                vdop,
            }))
        } else {
            Ok(None)
        }
    }
    /// Retrieves a slice containing the PRNs for satellites used in the fix in the GSA message.
    pub fn get_fix_satellites_prn(&self) -> &[u16] {
        &self.fix_sats_prn[..self.prn_array_size]
    }
}

/// Receiver mode of positioning.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FixType {
    /// No valid position is available.
    NoFix,
    /// The receiver has a 2D position fix
    Fix2D,
    /// The receiver has a 3D position fix
    Fix3D,
}

impl FixType {
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<FixType>, &'static str> {
        match input {
            Some("1") => Ok(Some(FixType::NoFix)),
            Some("2") => Ok(Some(FixType::Fix2D)),
            Some("3") => Ok(Some(FixType::Fix3D)),
            _ => Err("Wrong FixType indicator type!"),
        }
    }
}

#[test]
fn test_parse_fixtype() {
    assert_eq!(FixType::parse(Some("1")), Ok(Some(FixType::NoFix)));
    assert_eq!(FixType::parse(Some("2")), Ok(Some(FixType::Fix2D)));
    assert_eq!(FixType::parse(Some("3")), Ok(Some(FixType::Fix3D)));
    assert!(FixType::parse(Some("9")).is_err());
}
