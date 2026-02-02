//! Structures that describes coordinates that may be parsed from NMEA sentences.
use core::convert::TryFrom;

/// Earth hemisphere
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hemisphere {
    /// North
    North,
    /// South
    South,
    /// East
    East,
    /// West
    West,
}

/// Latitude as reported by receiver.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Latitude {
    /// Degrees
    pub degrees: u8,
    /// Minutes
    pub minutes: u8,
    /// Seconds. Precision depends on receiver.
    pub seconds: f32,
    /// Earth hemisphere. North or south.
    pub hemisphere: Hemisphere,
}

impl TryFrom<f32> for Latitude {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        TryFrom::try_from(value as f64)
    }
}

impl TryFrom<f64> for Latitude {
    type Error = &'static str;

    fn try_from(from: f64) -> Result<Self, Self::Error> {
        if from >= 90f64 || from <= -90f64 {
            Err("Latitude is not in range -90 to 90 degrees!")
        } else {
            let (value, hemisphere) = if from >= 0f64 {
                (from, Hemisphere::North)
            } else {
                (-from, Hemisphere::South)
            };
            let degrees = value as u8;
            let min_sec = (value - degrees as f64) * 60f64;
            let minutes = min_sec as u8;
            let seconds = ((min_sec - minutes as f64) * 60f64) as f32;
            Ok({
                Latitude {
                    degrees,
                    minutes,
                    seconds,
                    hemisphere,
                }
            })
        }
    }
}

impl Latitude {
    pub(crate) fn parse(
        coord: Option<&str>,
        hemi: Option<&str>,
    ) -> Result<Option<Self>, &'static str> {
        match (coord, hemi) {
            (Some(lat), Some(lat_hemi)) if lat.len() == 0 && lat_hemi.len() == 0 => Ok(None),
            (Some(lat), Some(lat_hemi)) => {
                if lat.len() < 4 {
                    return Err("Latitude field is too short!");
                }
                let hemisphere = match lat_hemi {
                    "N" => Hemisphere::North,
                    "S" => Hemisphere::South,
                    _ => return Err("Latitude hemisphere field has wrong format!"),
                };
                let degrees = lat[..2]
                    .parse::<u8>()
                    .map_err(|_| "Wrong latitude field format")?;
                let min_sec = lat[2..]
                    .parse::<f64>()
                    .map_err(|_| "Wrong latitude field format")?;
                let minutes = min_sec as u8;
                let seconds = ((min_sec - minutes as f64) * 60f64) as f32;
                Ok(Some(Latitude {
                    degrees,
                    minutes,
                    seconds,
                    hemisphere,
                }))
            }
            (None, Some(_)) => Err("Could not parse latitude from hemisphere only"),
            (Some(_), None) => Err("Could not parse latitude from coordinate only"),
            (None, None) => Ok(None),
        }
    }
    /// Return latitude in degrees f64 value. Negative for South hemisphere, positive for North.
    pub fn as_f64(&self) -> f64 {
        let result =
            self.degrees as f64 + (self.minutes as f64) / 60f64 + (self.seconds as f64) / 3600f64;
        match self.hemisphere {
            Hemisphere::North => result,
            Hemisphere::South => -result,
            Hemisphere::East => panic!("Wrong East hemisphere for latitude!"),
            Hemisphere::West => panic!("Wrong West hemisphere for latitude!"),
        }
    }
    /// Is north hemisphere
    pub fn is_north(&self) -> bool {
        self.hemisphere == Hemisphere::North
    }
    /// Is south hemisphere
    pub fn is_south(&self) -> bool {
        self.hemisphere == Hemisphere::South
    }
}

/// Longitude as reported by receiver.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Longitude {
    /// Degrees
    pub degrees: u8,
    /// Minutes
    pub minutes: u8,
    /// Second. Precision depends on receiver.
    pub seconds: f32,
    /// Earth hemisphere. East or West.
    pub hemisphere: Hemisphere,
}

impl TryFrom<f32> for Longitude {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        TryFrom::try_from(value as f64)
    }
}

impl TryFrom<f64> for Longitude {
    type Error = &'static str;

    fn try_from(from: f64) -> Result<Self, Self::Error> {
        if from >= 180f64 || from <= -180f64 {
            Err("Latitude is not in range -180 to 180 degrees!")
        } else {
            let (value, hemisphere) = if from >= 0f64 {
                (from, Hemisphere::East)
            } else {
                (-from, Hemisphere::West)
            };
            let degrees = value as u8;
            let min_sec = (value - degrees as f64) * 60f64;
            let minutes = min_sec as u8;
            let seconds = ((min_sec - minutes as f64) * 60f64) as f32;
            Ok({
                Longitude {
                    degrees,
                    minutes,
                    seconds,
                    hemisphere,
                }
            })
        }
    }
}

impl Longitude {
    pub(crate) fn parse(
        coord: Option<&str>,
        hemi: Option<&str>,
    ) -> Result<Option<Self>, &'static str> {
        match (coord, hemi) {
            (Some(lon), Some(lon_hemi)) if lon.len() == 0 && lon_hemi.len() == 0 => Ok(None),
            (Some(lon), Some(lon_hemi)) => {
                if lon.len() < 5 {
                    return Err("Longitude field is too short!");
                }
                let hemisphere = match lon_hemi {
                    "E" => Hemisphere::East,
                    "W" => Hemisphere::West,
                    _ => return Err("Longitude hemisphere field has wrong format!"),
                };
                let degrees = lon[..3]
                    .parse::<u8>()
                    .map_err(|_| "Wrong longitude field format")?;
                let min_sec = lon[3..]
                    .parse::<f64>()
                    .map_err(|_| "Wrong longitude field format")?;
                let minutes = min_sec as u8;
                let seconds = ((min_sec - minutes as f64) * 60f64) as f32;
                Ok(Some(Longitude {
                    degrees,
                    minutes,
                    seconds,
                    hemisphere,
                }))
            }
            (None, Some(_)) => Err("Could not parse longitude from hemisphere only"),
            (Some(_), None) => Err("Could not parse longitude from coordinate only"),
            (None, None) => Ok(None),
        }
    }
    /// Return longitude in degrees f64 value. Negative for West hemisphere, positive for East.
    pub fn as_f64(&self) -> f64 {
        let result =
            self.degrees as f64 + (self.minutes as f64) / 60f64 + (self.seconds as f64) / 3600f64;
        match self.hemisphere {
            Hemisphere::West => -result,
            Hemisphere::East => result,
            Hemisphere::North => panic!("Wrong North hemisphere for latitude!"),
            Hemisphere::South => panic!("Wrong South hemisphere for latitude!"),
        }
    }
    /// Is in west hemisphere
    pub fn is_west(&self) -> bool {
        self.hemisphere == Hemisphere::West
    }
    /// Is in east hemisphere
    pub fn is_east(&self) -> bool {
        self.hemisphere == Hemisphere::East
    }
}

/// Altitude reported by receiver typically in GGA sentence.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Altitude {
    /// Altitude in meters over ground.
    pub meters: f32,
}

impl Altitude {
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<Self>, &'static str> {
        match input {
            Some("") => Ok(None),
            Some(alt) => Ok(Some(Altitude {
                meters: alt
                    .parse::<f32>()
                    .map_err(|_| "Wrong altitude field format")?,
            })),
            _ => Ok(None),
        }
    }
}

/// Speed reported by receiver typically in RMC and VTG sentences.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Speed {
    knots: f32,
}

impl Speed {
    /// Speed from knots
    pub fn from_knots(speed: f32) -> Speed {
        Speed { knots: speed }
    }
    /// Speed from meters per second
    pub fn from_mps(speed: f32) -> Speed {
        Speed {
            knots: speed * 1.94384f32,
        }
    }
    /// Speed from miles per hour
    pub fn from_mph(speed: f32) -> Speed {
        Speed {
            knots: speed * 0.868976f32,
        }
    }
    /// Speed from kilometers per hour
    pub fn from_kph(speed: f32) -> Speed {
        Speed {
            knots: speed * 0.539957f32,
        }
    }
    /// Speed as knots
    pub fn as_knots(&self) -> f32 {
      self.knots
    }
    /// Speed as kilometers per hour
    pub fn as_kph(&self) -> f32 {
        self.knots * 1.852
    }
    /// Speed as miles per hour
    pub fn as_mph(&self) -> f32 {
        self.knots * 1.15078
    }
    /// Speed as meters per second
    pub fn as_mps(&self) -> f32 {
        self.knots * 0.514444
    }
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<Self>, &'static str> {
        match input {
            Some(speed) if speed.len() == 0 => Ok(None),
            Some(speed) => speed
                .parse::<f32>()
                .map_err(|_| "Wrong speed field format")
                .and_then(|knots| Ok(Some(Speed { knots }))),
            _ => Ok(None),
        }
    }
}

/// The course over ground.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Course {
    /// Course in degrees from North rotated clockwise.
    pub degrees: f32,
}

impl From<f32> for Course {
    fn from(value: f32) -> Self {
        Course { degrees: value }
    }
}

impl Course {
    pub(crate) fn parse(input: Option<&str>) -> Result<Option<Self>, &'static str> {
        match input {
            Some(course) if course.len() == 0 => Ok(None),
            Some(course) => course
                .parse::<f32>()
                .map_err(|_| "Wrong course field format")
                .and_then(|degrees| Ok(Some(Course { degrees }))),
            _ => Ok(None),
        }
    }
}

/// The course over ground calculated from True course and magnetic variation.
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagneticCourse {
    /// Course in degrees from Magnetic North Pole rotated clockwise.
    pub degrees: f32,
}

impl From<f32> for MagneticCourse {
    fn from(value: f32) -> Self {
        MagneticCourse { degrees: value }
    }
}

impl MagneticCourse {
    pub(crate) fn parse_from_str(input: Option<&str>) -> Result<Option<Self>, &'static str> {
        match input {
            Some(course) if course.len() == 0 => Ok(None),
            Some(course) => course
                .parse::<f32>()
                .map_err(|_| "Wrong course field format")
                .and_then(|degrees| Ok(Some(MagneticCourse { degrees }))),
            _ => Ok(None),
        }
    }
    pub(crate) fn parse_from_mvar_mdir(
        true_course: &Option<Course>,
        mvar: Option<&str>,
        mdir: Option<&str>,
    ) -> Result<Option<Self>, &'static str> {
        if let (Some(course), Some(variation), Some(direction)) = (true_course, mvar, mdir) {
            if variation.len() == 0 && direction.len() == 0 {
                Ok(None)
            } else {
                let magnetic = variation
                    .parse::<f32>()
                    .map_err(|_| "Wrong magnetic variation field format!")?;
                match direction {
                    "E" => Ok(Some(MagneticCourse {
                        degrees: course.degrees - magnetic,
                    })),
                    "W" => Ok(Some(MagneticCourse {
                        degrees: course.degrees + magnetic,
                    })),
                    _ => Err("Wrong direction field for magnetic variation"),
                }
            }
        } else {
            Ok(None)
        }
    }
}
