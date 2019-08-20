#![no_std]
#![warn(missing_docs)]
//! NMEA 0183 parser. Implemented most used sentences like RMC, VGT, GGA, GLL.
//! Parser do not use heap memory and relies only on `core`.
//!
//! You should instantiate `Parser` with `new` and than use methods like `parse_from_byte` or `parse_from_bytes`.
//! If parser accumulates enough data it will return `ParseResult` or `&str` that describing an error.
//!
//! You do not need to do any preprocessing such as split data to strings or NMEA sentences.
//!
//! # Examples
//!
//! If you could read a one byte at a time from the receiver you may use `parse_from_byte`:
//! ```
//! use nmea_0183::{Parser, ParseResult};
//!
//! let nmea = b"$GPGGA,145659.00,5956.695396,N,03022.454999,E,2,07,0.6,9.0,M,18.0,M,,*62\r\n$GPGGA,,,,,,,,,,,,,,*00\r\n";
//! let mut parser = Parser::new();
//! for b in &nmea[..] {
//!     if let Some(result) = parser.parse_from_byte(*b) {
//!         match result {
//!             Ok(ParseResult::GGA(Some(gga))) => { }, // Got GGA sentence
//!             Ok(ParseResult::GGA(None)) => { }, // Got GGA sentence without valid data, receiver ok but has no solution
//!             Ok(_) => {}, // Some other sentences..
//!             Err(e) => { } // Got parse error
//!         }
//!     }
//! }
//! ```
//!
//! If you read many bytes from receiver at once or want to parse NMEA log from text file you could use Iterator-style:
//! ```
//! use nmea_0183::{Parser, ParseResult};
//!
//! let nmea = b"$GPGGA,,,,,,,,,,,,,,*00\r\n$GPRMC,125504.049,A,5542.2389,N,03741.6063,E,0.06,25.82,200906,,,A*56\r\n";
//! let mut parser = Parser::new();
//!
//! for result in parser.parse_from_bytes(&nmea[..]) {
//!     match result {
//!         Ok(ParseResult::RMC(Some(rmc))) => { }, // Got RMC sentence
//!         Ok(ParseResult::GGA(None)) => { }, // Got GGA sentence without valid data, receiver ok but has no solution
//!         Ok(_) => {}, // Some other sentences..
//!         Err(e) => { } // Got parse error
//!     }
//! }
//!
//! ```
use core::convert::TryFrom;
use core::slice::Iter;

pub(crate) mod common;
pub mod coords;
pub mod datetime;
pub(crate) mod gga;
pub(crate) mod gll;
pub(crate) mod modes;
pub(crate) mod rmc;
pub(crate) mod vtg;

pub use gga::GPSQuality;
pub use gga::GGA;
pub use gll::GLL;
pub use modes::Mode;
pub use rmc::RMC;
pub use vtg::VTG;

/// Source of NMEA sentence like GPS, GLONASS or other.
#[derive(Debug, PartialEq)]
pub enum Source {
    /// USA Global Positioning System
    GPS,
    /// Russian Federation GLONASS
    GLONASS,
    /// European Union Gallileo
    Gallileo,
    /// China's Beidou
    Beidou,
    /// Global Navigation Sattelite System. Some combination of other systems. Depends on receiver model, receiver settings, etc..
    GNSS,
}

impl TryFrom<&str> for Source {
    type Error = &'static str;

    fn try_from(from: &str) -> Result<Self, &'static str> {
        match &from[0..2] {
            "GP" => Ok(Source::GPS),
            "GL" => Ok(Source::GLONASS),
            "GA" => Ok(Source::Gallileo),
            "BD" => Ok(Source::Beidou),
            "GN" => Ok(Source::GNSS),
            _ => Err("Source is not supported"),
        }
    }
}

/// The NMEA sentence parsing result.
/// Sentences with many null fields or sentences without valid data is also parsed and returned as None.
/// None ParseResult may be interpreted as working receiver but without valid data.
#[derive(Debug, PartialEq)]
pub enum ParseResult {
    /// The Recommended Minimum Sentence for any GNSS. Typically most used.
    RMC(Option<RMC>),
    /// The Geographic coordinates including altitude, GPS solution quality, DGPS usage information.
    GGA(Option<GGA>),
    /// The Geographic latitude ang longitude sentence with time of fix and the receiver state.
    GLL(Option<GLL>),
    /// The actual course and speed relative to the ground.
    VTG(Option<VTG>),
}

/// Parses NMEA sentences and stores intermediate parsing state.
/// Parser is tolerant for errors so you should not reinitialize it after errors.
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

struct ParserIterator<'a> {
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
    /// Constructs new Parser.
    pub fn new() -> Parser {
        Parser {
            buffer: [0u8; 79],
            buflen: 0,
            chksum: 0,
            expected_chksum: 0,
            parser_state: ParserState::WaitStart,
        }
    }
    /// Use parser state and bytes slice than returns Iterator that yield ['ParseResult'] or errors if has enough data for parsing.
    pub fn parse_from_bytes<'a>(
        &'a mut self,
        input: &'a [u8],
    ) -> impl Iterator<Item = Result<ParseResult, &'static str>> + 'a {
        ParserIterator::new(self, input)
    }

    /// Parse NMEA by one byte at a time. Returns Some if has enough data for parsing.
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
            "RMC" => Ok(ParseResult::RMC(RMC::parse(source, &mut iter)?)),
            "GGA" => Ok(ParseResult::GGA(GGA::parse(source, &mut iter)?)),
            "GLL" => Ok(ParseResult::GLL(GLL::parse(source, &mut iter)?)),
            "VTG" => Ok(ParseResult::VTG(VTG::parse(source, &mut iter)?)),
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
