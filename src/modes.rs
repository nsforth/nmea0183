#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum Status {
    Valid,
    NotValid,
}

impl Status {
    pub(crate) fn from_str(st: &str) -> Result<Status, &'static str> {
        match st {
            "A" => Ok(Status::Valid),
            "V" => Ok(Status::NotValid),
            _ => Err("Invalid status field!"),
        }
    }
}

/// Receiver mode of operation.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Autonomous mode without any external correction.
    Autonomous,
    /// Differential correction used.
    Differential,
    /// Estimated position from previous data and movement model.
    Estimated,
    /// Set by operator.
    Manual,
    /// Simulation mode.
    Simulator,
    /// Completely invalid state. Position data if present could not be used.
    NotValid,
}

impl Mode {
    /// Position data shoud be valid if true
    pub fn is_valid(&self) -> bool {
        match self {
            Mode::Autonomous => true,
            Mode::Differential => true,
            _ => false,
        }
    }
}

impl Mode {
    pub(crate) fn from_some_str(from: Option<&str>) -> Result<Self, &'static str> {
        match from {
            Some("A") => Ok(Mode::Autonomous),
            Some("D") => Ok(Mode::Differential),
            Some("E") => Ok(Mode::Estimated),
            Some("M") => Ok(Mode::Manual),
            Some("S") => Ok(Mode::Simulator),
            Some("N") => Ok(Mode::NotValid),
            None => Err("Mode field shoud not be null!"),
            Some("") => Err("Mode should not be empty string!"),
            _ => Err("Wrong mode character!"),
        }
    }
    pub(crate) fn from_some_str_or_status(
        from: Option<&str>,
        alternate: &Status,
    ) -> Result<Self, &'static str> {
        match from {
            Some("A") => Ok(Mode::Autonomous),
            Some("D") => Ok(Mode::Differential),
            Some("E") => Ok(Mode::Estimated),
            Some("M") => Ok(Mode::Manual),
            Some("S") => Ok(Mode::Simulator),
            Some("N") => Ok(Mode::NotValid),
            None => match alternate {
                Status::Valid => Ok(Mode::Autonomous),
                Status::NotValid => Ok(Mode::NotValid),
            },
            Some("") => Err("Mode should not be empty string!"),
            _ => Err("Wrong mode character!"),
        }
    }
}

#[test]
fn test_parse_status() {
    assert_eq!(Status::from_str("A"), Ok(Status::Valid));
    assert_eq!(Status::from_str("V"), Ok(Status::NotValid));
    assert_eq!(Status::from_str(""), Err("Invalid status field!"));
}

#[test]
fn test_parse_mode() {
    assert_eq!(Mode::from_some_str(Some("A")), Ok(Mode::Autonomous));
    assert_eq!(Mode::from_some_str(Some("D")), Ok(Mode::Differential));
    assert_eq!(Mode::from_some_str(Some("E")), Ok(Mode::Estimated));
    assert_eq!(Mode::from_some_str(Some("M")), Ok(Mode::Manual));
    assert_eq!(Mode::from_some_str(Some("S")), Ok(Mode::Simulator));
    assert_eq!(Mode::from_some_str(Some("N")), Ok(Mode::NotValid));
    assert!(Mode::from_some_str(None).is_err());
    assert!(Mode::from_some_str(Some("")).is_err());
    assert!(Mode::from_some_str(Some("abc")).is_err());
}

#[test]
fn test_parse_mode_or_status() {
    assert_eq!(
        Mode::from_some_str_or_status(Some("A"), &Status::Valid),
        Ok(Mode::Autonomous)
    );
    assert_eq!(
        Mode::from_some_str_or_status(Some("D"), &Status::Valid),
        Ok(Mode::Differential)
    );
    assert_eq!(
        Mode::from_some_str_or_status(Some("E"), &Status::NotValid),
        Ok(Mode::Estimated)
    );
    assert_eq!(
        Mode::from_some_str_or_status(Some("M"), &Status::NotValid),
        Ok(Mode::Manual)
    );
    assert_eq!(
        Mode::from_some_str_or_status(Some("S"), &Status::NotValid),
        Ok(Mode::Simulator)
    );
    assert_eq!(
        Mode::from_some_str_or_status(Some("N"), &Status::NotValid),
        Ok(Mode::NotValid)
    );
    assert_eq!(
        Mode::from_some_str_or_status(None, &Status::NotValid),
        Ok(Mode::NotValid)
    );
    assert!(Mode::from_some_str_or_status(Some(""), &Status::NotValid).is_err());
    assert!(Mode::from_some_str_or_status(Some("abc"), &Status::NotValid).is_err());
}
