use crate::common;
use crate::satellite::Satellite;

use crate::Source;
/// Satellites in views including the number of SVs in view, the PRN numbers, elevations, azimuths, and SNR values.
#[derive(Debug, PartialEq, Clone)]
pub struct GSV {
    /// The total number of GSV messages for the current data.
    pub total_messages_number: u8,
    /// The message number (1 to the total number of messages) for the current GSV sentence.
    pub message_number: u8,
    /// Total number of satellites in view.
    pub sat_in_view: u8,
    /// Information about first SV.
    pub sat_info_1: Option<Satellite>,
    /// Information about second SV.
    pub sat_info_2: Option<Satellite>,
    /// Information about third SV.
    pub sat_info_3: Option<Satellite>,
    /// Information about fourth SV.
    pub sat_info_4: Option<Satellite>,
}

impl GSV {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let total_messages_number = common::parse_u8(fields.next())?;
        let message_number = common::parse_u8(fields.next())?;
        let sat_in_view = common::parse_u8(fields.next())?;
        let mut sat_info_1 = None;
        let mut sat_info_2 = None;
        let mut sat_info_3 = None;
        let mut sat_info_4 = None;
        for sat_num in 1..=4 {
            if let Some(sat_info) = Satellite::parse(fields, source)? {
                match sat_num {
                    1 => sat_info_1 = Some(sat_info),
                    2 => sat_info_2 = Some(sat_info),
                    3 => sat_info_3 = Some(sat_info),
                    4 => sat_info_4 = Some(sat_info),
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        }

        if let (Some(total_messages_number), Some(message_number), Some(sat_in_view)) =
            (total_messages_number, message_number, sat_in_view)
        {
            Ok(Some(GSV {
                total_messages_number,
                message_number,
                sat_in_view,
                sat_info_1,
                sat_info_2,
                sat_info_3,
                sat_info_4,
            }))
        } else {
            Ok(None)
        }
    }
}
