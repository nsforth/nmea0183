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

#[test]
fn test_parse_u8() {
    assert_eq!(parse_u8(Some("")), Ok(None));
    assert_eq!(parse_u8(Some("123")), Ok(Some(123u8)));
    assert_eq!(
        parse_u8(Some("a123")),
        Err("Wrong unsigned int field format")
    );
    assert_eq!(
        parse_u8(Some("-123")),
        Err("Wrong unsigned int field format")
    );
    assert_eq!(
        parse_u8(Some("256")),
        Err("Wrong unsigned int field format")
    );
}

#[test]
fn test_parse_u16() {
    assert_eq!(parse_u16(Some("")), Ok(None));
    assert_eq!(parse_u16(Some("123")), Ok(Some(123u16)));
    assert_eq!(
        parse_u16(Some("a123")),
        Err("Wrong unsigned int field format")
    );
    assert_eq!(
        parse_u16(Some("-123")),
        Err("Wrong unsigned int field format")
    );
    assert_eq!(
        parse_u16(Some("70000")),
        Err("Wrong unsigned int field format")
    );
}

#[test]
fn test_parse_f32() {
    assert_eq!(parse_f32(Some("")), Ok(None));
    assert_eq!(parse_f32(Some("123.0")), Ok(Some(123.0f32)));
    assert_eq!(parse_f32(Some("a123.0")), Err("Wrong float field format"));
}
