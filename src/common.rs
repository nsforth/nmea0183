pub(crate) fn parse_u8(input: Option<&str>) -> Result<Option<u8>, &'static str> {
    match input {
        Some(s) if s.len() == 0 => Ok(None),
        Some(s) => s
            .parse::<u8>()
            .map_err(|_| "Wrong unsigned int field format")
            .and_then(|u| Ok(Some(u))),
        _ => Ok(None),
    }
}

pub(crate) fn parse_u16(input: Option<&str>) -> Result<Option<u16>, &'static str> {
    match input {
        Some(s) if s.len() == 0 => Ok(None),
        Some(s) => s
            .parse::<u16>()
            .map_err(|_| "Wrong unsigned int field format")
            .and_then(|u| Ok(Some(u))),
        _ => Ok(None),
    }
}

pub(crate) fn parse_f32(input: Option<&str>) -> Result<Option<f32>, &'static str> {
    match input {
        Some(s) if s.len() == 0 => Ok(None),
        Some(s) => s
            .parse::<f32>()
            .map_err(|_| "Wrong float field format")
            .and_then(|u| Ok(Some(u))),
        _ => Ok(None),
    }
}
