use crate::coords::{Course, MagneticCourse, Speed};
use crate::modes::Mode;
use crate::Source;

/// The actual course and speed relative to the ground.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VTG {
    /// Navigational system.
    pub source: Source,
    /// Course over ground. Some receivers do not reports it when no movement.
    pub course: Option<Course>,
    /// Magnetic course over ground (angle to magnetic North pole).
    pub magnetic: Option<MagneticCourse>,
    /// Speed over ground.
    pub speed: Speed,
    /// Receiver's mode of operation.
    pub mode: Mode,
}

impl VTG {
    pub(crate) fn parse<'a>(
        source: Source,
        fields: &mut core::str::Split<'a, char>,
    ) -> Result<Option<Self>, &'static str> {
        let course = Course::parse(fields.next())?;
        fields.next(); // Not needed true course marker field
        let magnetic = MagneticCourse::parse_from_str(fields.next())?;
        fields.next(); // Not needed magnetic course marker field
        let speed = Speed::parse(fields.next())?;
        fields.next(); // Not needed speed knots marker field
        fields.next(); // Not needed speed kph field
        fields.next(); // Not needed speed kph marker field
        let mode = Mode::from_some_str(fields.next())?;

        if let Some(speed) = speed {
            Ok(Some(VTG {
                source,
                course,
                magnetic,
                speed,
                mode,
            }))
        } else {
            Ok(None)
        }
    }
}
