use core::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Hemisphere {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub struct Latitude {
    pub degrees: u8,
    pub minutes: u8,
    pub seconds: f32,
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

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 90f64 || value <= -90f64 {
            Err("Latitude is not in range -90 to 90 degrees!")
        } else {
            let hemisphere = if value > 0f64 {
                Hemisphere::North
            } else {
                Hemisphere::South
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
    pub fn parse(coord: Option<&str>, hemi: Option<&str>) -> Result<Option<Self>, &'static str> {
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

    pub fn is_north(&self) -> bool {
        self.hemisphere == Hemisphere::North
    }

    pub fn is_south(&self) -> bool {
        self.hemisphere == Hemisphere::South
    }
}

#[derive(Debug, PartialEq)]
pub struct Longitude {
    pub degrees: u8,
    pub minutes: u8,
    pub seconds: f32,
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

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 180f64 || value <= -180f64 {
            Err("Latitude is not in range -180 to 180 degrees!")
        } else {
            let hemisphere = if value > 0f64 {
                Hemisphere::East
            } else {
                Hemisphere::West
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
    pub fn parse(coord: Option<&str>, hemi: Option<&str>) -> Result<Option<Self>, &'static str> {
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

    pub fn is_west(&self) -> bool {
        self.hemisphere == Hemisphere::West
    }

    pub fn is_east(&self) -> bool {
        self.hemisphere == Hemisphere::East
    }
}

#[derive(Debug, PartialEq)]
pub struct Altitude {
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

#[derive(Debug, PartialEq)]
pub struct Speed {
    knots: f32,
}

impl Speed {
    pub fn from_knots(speed: f32) -> Speed {
        Speed { knots: speed }
    }
    pub fn from_mps(speed: f32) -> Speed {
        Speed {
            knots: speed * 1.94384f32,
        }
    }
    pub fn from_mph(speed: f32) -> Speed {
        Speed {
            knots: speed * 0.868976f32,
        }
    }
    pub fn from_kph(speed: f32) -> Speed {
        Speed {
            knots: speed * 0.539957f32,
        }
    }
    pub fn as_kph(&self) -> f32 {
        self.knots * 1.852
    }
    pub fn as_mph(&self) -> f32 {
        self.knots * 1.15078
    }
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

#[derive(Debug, PartialEq)]
pub struct Course {
    degrees: f32,
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

#[derive(Debug, PartialEq)]
pub struct MagneticCourse {
    degrees: f32,
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
