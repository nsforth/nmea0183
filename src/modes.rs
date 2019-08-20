#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
