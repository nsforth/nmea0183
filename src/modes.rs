#[derive(Debug, PartialEq)]
pub enum Status {
    Valid,
    NotValid,
}

impl Status {
    pub fn is_valid(&self) -> bool {
        *self == Status::Valid
    }
    pub(crate) fn from_str(st: &str) -> Result<Status, &'static str> {
        match st {
            "A" => Ok(Status::Valid),
            "V" => Ok(Status::NotValid),
            _ => Err("Invalid status field!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Autonomous,
    Differential,
    Estimated,
    Manual,
    Simulator,
    NotValid,
}

impl Mode {
    pub fn is_valid(&self) -> bool {
        match self {
            Mode::Autonomous => true,
            Mode::Differential => true,
            _ => false,
        }
    }
}

impl Mode {
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
