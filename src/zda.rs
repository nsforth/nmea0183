use crate::datetime::{Date, DateTime, Time};
use crate::{common, Source};

/// Geographic latitude ang longitude sentence with time of fix and receiver state.
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ZDA {
    /// Navigational system.
    pub source: Source,
    /// Date and time in UTC. If receiver uses local time see offset hour and minutes.
    pub datetime: DateTime,
    /// Offset in hours from UTC.
    pub offset_hours: Option<i8>,
    /// Offset in minutes from UTC,
    pub offset_minutes: Option<u8>,
}

impl ZDA {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let time = Time::parse_from_hhmmss(fields.next())?;
        let date = ZDA::parse_zda_date(fields)?;
        let offset_hours = common::parse_i8(fields.next())?;
        let offset_minutes = common::parse_u8(fields.next())?;

        if let (Some(time), Some(date), offset_hours, offset_minutes) =
            (time, date, offset_hours, offset_minutes)
        {
            Ok(Some(ZDA {
                source,
                datetime: DateTime { date, time },
                offset_hours,
                offset_minutes,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_zda_date<'a>(
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Date>, &'static str> {
        let day = common::parse_u8(fields.next())?;
        let month = common::parse_u8(fields.next())?;
        let year = common::parse_u16(fields.next())?;
        if let (Some(day), Some(month), Some(year)) = (day, month, year) {
            Ok(Some(Date { day, month, year }))
        } else {
            Ok(None)
        }
    }
}
