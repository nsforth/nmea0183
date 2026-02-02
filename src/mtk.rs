use crate::Source;
use core::convert::TryFrom;

/// MTK NMEA packet type
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MTKPacketType {
    /// Related to Spoofing and Jamming detection
    SPF,
}

impl TryFrom<&str> for MTKPacketType {
    type Error = &'static str;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "SPF" => Ok(MTKPacketType::SPF),
            _ => Err("Unsupported MTKPacketType."),
        }
    }
}

/// Related to Spoofing and Jamming detection .
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PMTKSPF {
    /// Navigational system.
    pub source: Source,
    /// GPS Jamming status
    pub jamming_status: JammingStatus,
}

impl PMTKSPF {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let jamming_status = JammingStatus::parse(fields.next())?;
        if let Some(jamming_status) = jamming_status {
            Ok(Some(PMTKSPF {
                source,
                jamming_status,
            }))
        } else {
            Ok(None)
        }
    }
}

/// Status of gps Jamming
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JammingStatus {
    /// No Jamming
    Healthy,
    /// Instantaneous jamming
    Warning,
    /// Continuous jamming
    Critical,
}

impl JammingStatus {
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<JammingStatus>, &'static str> {
        match input {
            Some("1") => Ok(Some(JammingStatus::Healthy)),
            Some("2") => Ok(Some(JammingStatus::Warning)),
            Some("3") => Ok(Some(JammingStatus::Critical)),
            Some("") => Ok(None),
            None => Ok(None),
            _ => Err("Wrong JammingStatus indicator type!"),
        }
    }
}

#[test]
fn test_parse_jammingstatus() {
    assert_eq!(
        JammingStatus::parse(Some("1")),
        Ok(Some(JammingStatus::Healthy))
    );
    assert_eq!(
        JammingStatus::parse(Some("2")),
        Ok(Some(JammingStatus::Warning))
    );
    assert_eq!(
        JammingStatus::parse(Some("3")),
        Ok(Some(JammingStatus::Critical))
    );
    assert_eq!(JammingStatus::parse(Some("")), Ok(None));
    assert_eq!(JammingStatus::parse(None), Ok(None));
    assert!(JammingStatus::parse(Some("0")).is_err());
}
