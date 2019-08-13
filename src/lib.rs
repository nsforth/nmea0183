//#![no_std]
use core::convert::TryFrom;

pub mod datetime;
pub mod rmc;

#[derive(Debug, PartialEq)]
pub enum Source {
    GPS,
    GLONASS,
    GALLILEO,
    GNSS,
}

impl TryFrom<&str> for Source {
    type Error = &'static str;

    fn try_from(from: &str) -> Result<Self, &'static str> {
        match &from[0..2] {
            "GP" => Ok(Source::GPS),
            "GL" => Ok(Source::GLONASS),
            "GA" => Ok(Source::GALLILEO),
            "GN" => Ok(Source::GNSS),
            _ => Err("Source is not supported"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseResult {
    RMC(Option<rmc::RMC>),
    UnsupportedSentence,
    InProgress,
}

pub struct Parser {
    buffer: [u8; 79],
    buflen: usize,
    chksum: u8,
    expected_chksum: u8,
    parser_state: ParserState,
}

#[derive(Debug)]
enum ParserState {
    WaitStart,
    ReadUntilChkSum,
    ChkSumUpper,
    ChkSumLower,
    WaitCR,
    WaitLF,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            buffer: [0u8; 79],
            buflen: 0,
            chksum: 0,
            expected_chksum: 0,
            parser_state: ParserState::WaitStart,
        }
    }

    pub fn parse(&mut self, input: &[u8]) -> Result<ParseResult, &'static str> {
        for b in input {
            let symbol = *b;
            match self.parser_state {
                ParserState::WaitStart if symbol == b'$' => {
                    self.parser_state = ParserState::ReadUntilChkSum;
                    self.buflen = 0;
                    self.chksum = 0;
                }
                ParserState::WaitStart if symbol != b'$' => continue,
                ParserState::ReadUntilChkSum if symbol != b'*' => {
                    if self.buffer.len() < self.buflen {
                        return Err("NMEA sentence is too long!");
                    }
                    self.buffer[self.buflen] = symbol;
                    self.buflen += 1;
                    self.chksum = self.chksum ^ symbol;
                }
                ParserState::ReadUntilChkSum if symbol == b'*' => {
                    self.parser_state = ParserState::ChkSumUpper;
                }
                ParserState::ChkSumUpper => {
                    self.expected_chksum = parse_hex_halfbyte(symbol)?;
                    self.parser_state = ParserState::ChkSumLower;
                }
                ParserState::ChkSumLower => {
                    self.expected_chksum =
                        (self.expected_chksum << 4) | parse_hex_halfbyte(symbol)?;
                    if self.expected_chksum != self.chksum {
                        self.parser_state = ParserState::WaitStart;
                        return Err("Checksum error!");
                    } else {
                        self.parser_state = ParserState::WaitCR;
                    }
                }
                ParserState::WaitCR if symbol == b'\r' => {
                    self.parser_state = ParserState::WaitLF;
                }
                ParserState::WaitLF if symbol == b'\n' => {
                    self.parser_state = ParserState::WaitStart;
                    return self.parse_sentences();
                }
                _ => {
                    self.parser_state = ParserState::WaitStart;
                    return Err("NMEA format error!");
                }
            }
        }
        Ok(ParseResult::InProgress)
    }

    fn parse_sentences(&self) -> Result<ParseResult, &'static str> {
        let input = from_ascii(&self.buffer[..self.buflen])?;
        let mut iter = input.split(',');
        let sentence_field = iter
            .next()
            .ok_or("Sentence type not found but mandatory!")?;
        if sentence_field.len() < 5 {
            return Err("Sentence field is too small. Must be 5 chars at least!");
        }
        let source = Source::try_from(sentence_field)?;
        match &sentence_field[2..5] {
            "RMC" => Ok(ParseResult::RMC(rmc::RMC::parse_rmc(source, &mut iter)?)),
            _ => Ok(ParseResult::UnsupportedSentence),
        }
    }
}

fn from_ascii(bytes: &[u8]) -> Result<&str, &'static str> {
    if bytes.iter().all(|b| *b < 128) {
        Ok(unsafe { core::str::from_utf8_unchecked(bytes) })
    } else {
        Err("Not an ascii!")
    }
}

fn parse_hex_halfbyte(symbol: u8) -> Result<u8, &'static str> {
    if symbol >= b'0' && symbol <= b'9' {
        return Ok(symbol - b'0');
    }
    if symbol >= b'A' && symbol <= b'F' {
        return Ok(symbol - b'A' + 10);
    }
    Err("Invalid HEX character.")
}

#[test]
fn test_correct_nmea_block() {
    let mut p = Parser::new();
    let b = b"$GPVTG,089.0,T,,,15.2,N,,*7F\r\n";
    assert_eq!(p.parse(&b[..]), Ok(ParseResult::UnsupportedSentence));
}

#[test]
fn test_correct_rmc() {
    let mut p = Parser::new();
    let b = b"$GPRMC,125504.049,A,5542.2389,N,03741.6063,E,0.06,25.82,200906,,,A*56\r\n";
    assert_eq!(
        p.parse(&b[..]),
        Ok(ParseResult::RMC(Some(rmc::RMC {
            source: Source::GPS,
            datetime: datetime::DateTime {
                date: datetime::Date {
                    day: 20,
                    month: 9,
                    year: 2006
                },
                time: datetime::Time {
                    hours: 12,
                    minutes: 55,
                    seconds: 4.049
                }
            },
            latitude: 55.703981666666664,
            longitude: 37.69343833333333,
            speed: 0.06,
            course: 25.82,
            magnetic: None,
            mode: rmc::RMCMode::Autonomous
        })))
    );
}

#[test]
fn test_correct_rmc2() {
    let mut p = Parser::new();
    let b = b"$GPRMC,113650.0,A,5548.607,N,03739.387,E,000.01,255.6,210403,08.7,E*69\r\n";
    assert_eq!(
        p.parse(&b[..]),
        Ok(ParseResult::RMC(Some(rmc::RMC {
            source: Source::GPS,
            datetime: datetime::DateTime {
                date: datetime::Date {
                    day: 21,
                    month: 4,
                    year: 2003
                },
                time: datetime::Time {
                    hours: 11,
                    minutes: 36,
                    seconds: 50.0
                }
            },
            latitude: 55.810116666666666,
            longitude: 37.65645,
            speed: 0.01,
            course: 255.6,
            magnetic: Some(8.7),
            mode: rmc::RMCMode::Autonomous
        })))
    );
}
