use crate::common;
use crate::satellite::Satellite;
use crate::Source;
const MAX_SATELLITES_PER_MESSAGE: usize = 4;
/// Satellites in views including the number of SVs in view, the PRN numbers, elevations, azimuths, and SNR values.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GSV {
    /// Navigational system.
    pub source: Source,
    /// The total number of GSV messages for the current data.
    pub total_messages_number: u8,
    /// The message number (1 to the total number of messages) for the current GSV sentence.
    pub message_number: u8,
    /// Total number of satellites in view.
    pub sat_in_view: u8,
    /// Array of satellite information.
    satellites: [Satellite; MAX_SATELLITES_PER_MESSAGE],
    /// The actual number of satellites in the array.
    satellite_array_size: usize,
}

impl GSV {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let total_messages_number = common::parse_u8(fields.next())?;
        let message_number = common::parse_u8(fields.next())?;
        let sat_in_view = common::parse_u8(fields.next())?;
        let mut satellites: [Satellite; MAX_SATELLITES_PER_MESSAGE] = Default::default();
        let mut satellite_array_size = 0;

        for satellite in satellites.iter_mut() {
            if let Some(parsed_satellite) = Satellite::parse(fields)? {
                *satellite = parsed_satellite;
                satellite_array_size += 1;
            } else {
                break;
            }
        }

        if let (Some(total_messages_number), Some(message_number), Some(sat_in_view)) =
            (total_messages_number, message_number, sat_in_view)
        {
            Ok(Some(GSV {
                source,
                total_messages_number,
                message_number,
                sat_in_view,
                satellites,
                satellite_array_size,
            }))
        } else {
            Ok(None)
        }
    }
    /// Retrieves a slice containing in view satellites information present in the GSV message.
    pub fn get_in_view_satellites(&self) -> &[Satellite] {
        &self.satellites[..self.satellite_array_size]
    }
}
