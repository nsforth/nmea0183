#![no_std]
use core::convert::TryFrom;
use core::slice::Iter;

pub mod coords;
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

pub struct ParserIterator<'a> {
    parser: &'a mut Parser,
    input: Iter<'a, u8>,
}

impl ParserIterator<'_> {
    fn new<'a>(p: &'a mut Parser, inp: &'a [u8]) -> ParserIterator<'a> {
        ParserIterator {
            parser: p,
            input: inp.iter(),
        }
    }
}

impl Iterator for ParserIterator<'_> {
    type Item = Result<ParseResult, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(b) = self.input.next() {
            let symbol = *b;
            if let Some(r) = self.parser.parse_from_byte(symbol) {
                return Some(r);
            }
        }
        None
    }
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

    pub fn parse_from_bytes<'a>(
        &'a mut self,
        input: &'a [u8],
    ) -> impl Iterator<Item = Result<ParseResult, &'static str>> + 'a {
        ParserIterator::new(self, input)
    }

    pub fn parse_from_byte(&mut self, symbol: u8) -> Option<Result<ParseResult, &'static str>> {
        let (new_state, result) = match self.parser_state {
            ParserState::WaitStart if symbol == b'$' => {
                self.buflen = 0;
                self.chksum = 0;
                (ParserState::ReadUntilChkSum, None)
            }
            ParserState::WaitStart if symbol != b'$' => (ParserState::WaitStart, None),
            ParserState::ReadUntilChkSum if symbol != b'*' => {
                if self.buffer.len() < self.buflen {
                    (
                        ParserState::WaitStart,
                        Some(Err("NMEA sentence is too long!")),
                    )
                } else {
                    self.buffer[self.buflen] = symbol;
                    self.buflen += 1;
                    self.chksum = self.chksum ^ symbol;
                    (ParserState::ReadUntilChkSum, None)
                }
            }
            ParserState::ReadUntilChkSum if symbol == b'*' => (ParserState::ChkSumUpper, None),
            ParserState::ChkSumUpper => match parse_hex_halfbyte(symbol) {
                Ok(s) => {
                    self.expected_chksum = s;
                    (ParserState::ChkSumLower, None)
                }
                Err(e) => (ParserState::WaitStart, Some(Err(e))),
            },
            ParserState::ChkSumLower => match parse_hex_halfbyte(symbol) {
                Ok(s) => {
                    if ((self.expected_chksum << 4) | s) != self.chksum {
                        (ParserState::WaitStart, Some(Err("Checksum error!")))
                    } else {
                        (ParserState::WaitCR, None)
                    }
                }
                Err(e) => (ParserState::WaitStart, Some(Err(e))),
            },
            ParserState::WaitCR if symbol == b'\r' => (ParserState::WaitLF, None),
            ParserState::WaitLF if symbol == b'\n' => {
                (ParserState::WaitStart, Some(self.parse_sentences()))
            }
            _ => (ParserState::WaitStart, Some(Err("NMEA format error!"))),
        };
        self.parser_state = new_state;
        return result;
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
            _ => Err("Unsupported sentence type"),
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
fn test_correct_but_unsupported_nmea_block() {
    let mut p = Parser::new();
    let sentence = b"$GPVTG,089.0,T,,,15.2,N,,*7F\r\n";
    let mut parsed = false;
    for b in sentence.iter() {
        let r = p.parse_from_byte(*b);
        if r.is_some() {
            assert_eq!(r.unwrap(), Err("Unsupported sentence type"));
            parsed = true;
            break;
        }
    }
    if !parsed {
        panic!("Parser failed to parse correct block!");
    }
}

#[test]
fn test_correct_rmc() {
    let mut p = Parser::new();
    let sentence = b"$GPRMC,125504.049,A,5542.2389,N,03741.6063,E,0.06,25.82,200906,,,A*56\r\n";
    let mut parsed = false;
    for b in sentence.iter() {
        let r = p.parse_from_byte(*b);
        if r.is_some() {
            assert_eq!(
                r.unwrap(),
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
                    latitude: TryFrom::try_from(55.703981666666664).unwrap(),
                    longitude: TryFrom::try_from(37.69343833333333).unwrap(),
                    speed: coords::Speed::from_knots(0.06),
                    course: 25.82,
                    magnetic: None,
                    mode: rmc::RMCMode::Autonomous
                })))
            );
            parsed = true;
            break;
        }
    }
    if !parsed {
        panic!("Parser failed to parse correct block!");
    }
}

#[test]
fn test_correct_rmc2() {
    let mut p = Parser::new();
    let sentence = b"$GPRMC,113650.0,A,5548.607,N,03739.387,E,000.01,255.6,210403,08.7,E*69\r\n";
    let mut parsed = false;
    for b in sentence.iter() {
        let r = p.parse_from_byte(*b);
        if r.is_some() {
            assert_eq!(
                r.unwrap(),
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
                    latitude: TryFrom::try_from(55.810116666666666).unwrap(),
                    longitude: TryFrom::try_from(37.65645).unwrap(),
                    speed: coords::Speed::from_knots(0.01),
                    course: 255.6,
                    magnetic: Some(8.7),
                    mode: rmc::RMCMode::Autonomous
                })))
            );
            parsed = true;
            break;
        }
    }
    if !parsed {
        panic!("Parser failed to parse correct block!");
    }
}

#[test]
fn test_parser_iterator() {
    let mut p = Parser::new();
    let b = b"$GPRMC,125504.049,A,5542.2389,N,03741.6063,E,0.06,25.82,200906,,,A*56\r\n";
    {
        let mut iter = p.parse_from_bytes(&b[..]);
        assert_eq!(
            iter.next().unwrap(),
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
                latitude: TryFrom::try_from(55.703981666666664).unwrap(),
                longitude: TryFrom::try_from(37.69343833333333).unwrap(),
                speed: coords::Speed::from_knots(0.06),
                course: 25.82,
                magnetic: None,
                mode: rmc::RMCMode::Autonomous
            })))
        );
    }
    let b1 = b"$GPRMC,125504.049,A,5542.2389,N";
    {
        let mut iter = p.parse_from_bytes(&b1[..]);
        assert!(iter.next().is_none());
    }
    let b2 = b",03741.6063,E,0.06,25.82,200906,,,";
    {
        let mut iter = p.parse_from_bytes(&b2[..]);
        assert!(iter.next().is_none());
    }
    let b3 = b"A*56\r\n";
    {
        let mut iter = p.parse_from_bytes(&b3[..]);
        assert_eq!(
            iter.next().unwrap(),
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
                latitude: TryFrom::try_from(55.703981666666664).unwrap(),
                longitude: TryFrom::try_from(37.69343833333333).unwrap(),
                speed: coords::Speed::from_knots(0.06),
                course: 25.82,
                magnetic: None,
                mode: rmc::RMCMode::Autonomous
            })))
        );
        assert!(iter.next().is_none());
    }
}
