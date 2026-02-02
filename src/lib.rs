#![no_std]
#![warn(missing_docs)]
//! NMEA 0183 parser. Implemented most used sentences like RMC, VTG, GGA, GLL.
//! Parser do not use heap memory and relies only on `core`.
//!
//! You should instantiate [Parser](struct.Parser.html) with [new](struct.Parser.html#method.new) and than use methods like [parse_from_byte](struct.Parser.html#method.parse_from_bytes) or [parse_from_bytes](struct.Parser.html#method.parse_from_bytes).
//! If parser accumulates enough data it will return [ParseResult](enum.ParseResult.html) on success or `&str` that describing an error.
//!
//! You do not need to do any preprocessing such as split data to strings or NMEA sentences.
//!
//! # Examples
//!
//! If you could read a one byte at a time from the receiver you may use `parse_from_byte`:
//! ```rust
//! use nmea0183::{Parser, ParseResult};
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
//! ```rust
//! use nmea0183::{Parser, ParseResult};
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
//!
//! It is possible to ignore some sentences or sources. You can set filter on [Parser](struct.Parser.html) like so:
//! ```rust
//! use nmea0183::{Parser, ParseResult, Sentence, Source};
//!
//! let parser_only_gps_gallileo = Parser::new()
//!     .source_filter(Source::GPS | Source::Gallileo);
//! let parser_only_rmc_gga_gps = Parser::new()
//!     .source_only(Source::GPS)
//!     .sentence_filter(Sentence::RMC | Sentence::GGA);
//! ```
//!
//! # Panics
//!
//! Should not panic. If so please report issue on project page.
//!
//! # Errors
//!
//! `Unsupported sentence type.` - Got currently not supported sentence.
//!
//! `Checksum error!` - Sentence has wrong checksum, possible data corruption.
//!
//! `Source is not supported!` - Unknown source, new sattelite system is launched? :)
//!
//! `NMEA format error!` - Possible data corruption. Parser drops all accumulated data and starts seek new sentences.
//!
//! It's possible to got other very rare error messages that relates to protocol errors. Receivers nowadays mostly do not violate NMEA specs.
//!
//! # Planned features
//!
//! GSA and GSV parsing.
//!
use core::convert::TryFrom;
use core::ops::BitOr;
use core::slice::Iter;
pub(crate) mod common;
pub mod coords;
pub mod datetime;
pub mod satellite;

pub(crate) mod gga;
pub(crate) mod gsa;
pub(crate) mod gsv;

pub(crate) mod gll;
pub(crate) mod modes;
#[cfg(feature = "mtk")]
pub(crate) mod mtk;
pub(crate) mod rmc;
pub(crate) mod vtg;
pub(crate) mod zda;
#[cfg(feature = "txt")]
pub(crate) mod txt;

pub use gga::GPSQuality;
pub use gga::GGA;
pub use gll::GLL;
pub use gsa::FixType;
pub use gsa::GSA;
pub use gsv::GSV;
pub use modes::Mode;
#[cfg(feature = "mtk")]
pub use mtk::JammingStatus;
#[cfg(feature = "mtk")]
pub use mtk::MTKPacketType;
#[cfg(feature = "mtk")]
pub use mtk::PMTKSPF;
pub use rmc::RMC;
pub use vtg::VTG;
pub use zda::ZDA;
#[cfg(feature = "txt")]
pub use txt::{TXT, MessageType};

/// Source of NMEA sentence like GPS, GLONASS or other.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    /// USA Global Positioning System
    GPS = 0b1,
    /// Russian Federation GLONASS
    GLONASS = 0b10,
    /// European Union Gallileo
    Gallileo = 0b100,
    /// China's Beidou
    Beidou = 0b1000,
    /// Global Navigation Sattelite System. Some combination of other systems. Depends on receiver model, receiver settings, etc..
    GNSS = 0b10000,
    /// Quasi-Zenith Satellite System
    QZSS = 0b100000,
    #[cfg(feature = "mtk")]
    /// MediaTek NMEA packet protocol
    MTK = 0b1000000,
}

/// Mask for Source filter in Parser.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SourceMask {
    mask: u32,
}

impl SourceMask {
    fn is_masked(&self, source: Source) -> bool {
        source as u32 & self.mask == 0
    }
}

impl Default for SourceMask {
    fn default() -> Self {
        SourceMask {
            mask: u32::max_value(),
        }
    }
}

impl BitOr for Source {
    type Output = SourceMask;
    fn bitor(self, rhs: Self) -> Self::Output {
        SourceMask {
            mask: self as u32 | rhs as u32,
        }
    }
}

impl BitOr<Source> for SourceMask {
    type Output = Self;
    fn bitor(self, rhs: Source) -> Self {
        SourceMask {
            mask: self.mask | rhs as u32,
        }
    }
}

impl TryFrom<&str> for Source {
    type Error = &'static str;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match &from[0..2] {
            "GP" => Ok(Source::GPS),
            "GL" => Ok(Source::GLONASS),
            "GA" => Ok(Source::Gallileo),
            "BD" => Ok(Source::Beidou),
            "GN" => Ok(Source::GNSS),
            "GQ" => Ok(Source::QZSS),
            #[cfg(feature = "mtk")]
            "PM" => Ok(Source::MTK),
            source => {
                #[cfg(feature = "defmt")]
                defmt::error!("Source is not supported: {}", source);
                Err("Source is not supported!")
            }
        }
    }
}

/// Various kinds of NMEA sentence like RMC, VTG or other. Used for filter by sentence type in Parser.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sentence {
    /// Recommended minimum sentence.
    RMC = 0b1,
    /// Course over ground.
    VTG = 0b10,
    /// Geographic coordinates including altitude, GPS solution quality, DGPS usage information.
    GGA = 0b100,
    /// Geographic latitude ang longitude sentence with time of fix and receiver state.
    GLL = 0b1000,
    #[cfg(feature = "mtk")]
    /// MTK properitary messages.
    PMTK = 0b10000,
    /// Satellites in views.
    GSV = 0b100000,
    /// GPS DOP and active satellites.
    GSA = 0b1000000,
    /// Current Date and Time.
    ZDA = 0b10000000,
    #[cfg(feature = "txt")]
    /// Text message from receiver.
    TXT = 0b100000000,
}

impl TryFrom<&str> for Sentence {
    type Error = &'static str;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "RMC" => Ok(Sentence::RMC),
            "GGA" => Ok(Sentence::GGA),
            "GLL" => Ok(Sentence::GLL),
            "VTG" => Ok(Sentence::VTG),
            "GSV" => Ok(Sentence::GSV),
            #[cfg(feature = "mtk")]
            "PMTK" => Ok(Sentence::PMTK),
            "GSA" => Ok(Sentence::GSA),
            "ZDA" => Ok(Sentence::ZDA),
            #[cfg(feature = "txt")]
            "TXT" => Ok(Sentence::TXT),

            sentence => {
                #[cfg(feature = "defmt")]
                defmt::error!("Unsupported NMEA sentence type: {}", sentence);
                Err("Unsupported sentence type.")
            }
        }
    }
}

/// Mask for Sentence filter in Parser.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SentenceMask {
    mask: u32,
}

impl SentenceMask {
    fn is_masked(&self, sentence: Sentence) -> bool {
        sentence as u32 & self.mask == 0
    }
}

impl Default for SentenceMask {
    fn default() -> Self {
        SentenceMask {
            mask: u32::max_value(),
        }
    }
}

impl BitOr for Sentence {
    type Output = SentenceMask;
    fn bitor(self, rhs: Self) -> Self::Output {
        SentenceMask {
            mask: self as u32 | rhs as u32,
        }
    }
}

impl BitOr<Sentence> for SentenceMask {
    type Output = Self;
    fn bitor(self, rhs: Sentence) -> Self {
        SentenceMask {
            mask: self.mask | rhs as u32,
        }
    }
}

/// The NMEA sentence parsing result.
/// Sentences with many null fields or sentences without valid data is also parsed and returned as None.
/// None ParseResult may be interpreted as working receiver but without valid data.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseResult {
    /// The Recommended Minimum Sentence for any GNSS. Typically most used.
    RMC(Option<RMC>),
    /// The Geographic coordinates including altitude, GPS solution quality, DGPS usage information.
    GGA(Option<GGA>),
    /// The Geographic latitude ang longitude sentence with time of fix and the receiver state.
    GLL(Option<GLL>),
    /// The actual course and speed relative to the ground.
    VTG(Option<VTG>),
    /// The satellites in views including the number of SVs in view, the PRN numbers, elevations, azimuths, and SNR values.
    GSV(Option<GSV>),
    #[cfg(feature = "mtk")]
    /// The MTK properitary messages.
    PMTK(Option<PMTKSPF>),
    /// The GPS DOP and active satellites. Provides information about the DOP and the active satellites used for the current fix.
    GSA(Option<GSA>),
    /// Current Date and Time.
    ZDA(Option<ZDA>),
    #[cfg(feature = "txt")]
    /// Text message from receiver.
    TXT(Option<TXT>),
}

#[cfg(feature = "strict")]
/// Maximum allowed sentence length, according to NMEA 183 docs should be not more than 79 chars. Disable strict feature to parse up to 120 chars.
pub const MAX_SENTENCE_LENGTH: usize = 79usize;

#[cfg(not(feature = "strict"))]
/// Maximum allowed sentence length.
pub const MAX_SENTENCE_LENGTH: usize = 120usize;

/// Parses NMEA sentences and stores intermediate parsing state.
/// Parser is tolerant for errors so you should not reinitialize it after errors.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Parser {
    buffer: [u8; MAX_SENTENCE_LENGTH],
    buflen: usize,
    chksum: u8,
    expected_chksum: u8,
    parser_state: ParserState,
    source_mask: SourceMask,
    sentence_mask: SentenceMask,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum ParserState {
    WaitStart,
    ReadUntilChkSum,
    ChkSumUpper,
    ChkSumLower,
    WaitCR,
    WaitLF,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
            buffer: [0u8; MAX_SENTENCE_LENGTH],
            buflen: 0,
            chksum: 0,
            expected_chksum: 0,
            parser_state: ParserState::WaitStart,
            source_mask: Default::default(),
            sentence_mask: Default::default(),
        }
    }
    /// Accepts only that [source](enum.Source.html)
    pub fn source_only(mut self, source: Source) -> Self {
        self.source_mask = SourceMask {
            mask: source as u32,
        };
        self
    }
    /// Ignore all [sources](enum.Source.html) except given.
    pub fn source_filter(mut self, source_mask: SourceMask) -> Self {
        self.source_mask = source_mask;
        self
    }
    /// Accepts only that [sentence](enum.Sentence.html)
    pub fn sentence_only(mut self, sentence: Sentence) -> Self {
        self.sentence_mask = SentenceMask {
            mask: sentence as u32,
        };
        self
    }
    /// Ignore all [sentences](enum.Sentence.html) except given.
    pub fn sentence_filter(mut self, sentence_mask: SentenceMask) -> Self {
        self.sentence_mask = sentence_mask;
        self
    }
    /// Use parser state and bytes slice than returns Iterator that yield [ParseResult](enum.ParseResult.html) or errors if has enough data for parsing.
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
                if self.buffer.len() <= self.buflen {
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
                (ParserState::WaitStart, self.parse_sentence().transpose())
            }
            _ => (ParserState::WaitStart, Some(Err("NMEA format error!"))),
        };
        self.parser_state = new_state;
        return result;
    }

    fn parse_sentence(&self) -> Result<Option<ParseResult>, &'static str> {
        let input = from_ascii(&self.buffer[..self.buflen])?;
        let mut iter = input.split(',');
        let sentence_field = iter
            .next()
            .ok_or("Sentence type not found but mandatory!")?;
        if sentence_field.len() < 5 {
            return Err("Sentence field is too small. Must be 5 chars at least!");
        }
        let source = Source::try_from(sentence_field)?;
        if self.source_mask.is_masked(source) {
            return Ok(None);
        }

        let sentence = match source {
            #[cfg(feature = "mtk")]
            Source::MTK => Sentence::try_from(&sentence_field[0..4])?,
            _ => Sentence::try_from(&sentence_field[2..5])?,
        };

        if self.sentence_mask.is_masked(sentence) {
            return Ok(None);
        }
        match sentence {
            Sentence::RMC => Ok(Some(ParseResult::RMC(RMC::parse(source, &mut iter)?))),
            Sentence::GGA => Ok(Some(ParseResult::GGA(GGA::parse(source, &mut iter)?))),
            Sentence::GLL => Ok(Some(ParseResult::GLL(GLL::parse(source, &mut iter)?))),
            Sentence::VTG => Ok(Some(ParseResult::VTG(VTG::parse(source, &mut iter)?))),
            Sentence::GSV => Ok(Some(ParseResult::GSV(GSV::parse(source, &mut iter)?))),
            Sentence::GSA => Ok(Some(ParseResult::GSA(GSA::parse(source, &mut iter)?))),
            Sentence::ZDA => Ok(Some(ParseResult::ZDA(ZDA::parse(source, &mut iter)?))),
            #[cfg(feature = "txt")]
            Sentence::TXT => Ok(Some(ParseResult::TXT(TXT::parse(source, &mut iter)?))),

            #[cfg(feature = "mtk")]
            Sentence::PMTK => {
                if sentence_field.len() < 7 {
                    return Err("PMTK Sentence field is too small. Must be 7 chars at least!");
                }
                match MTKPacketType::try_from(&sentence_field[4..7])? {
                    MTKPacketType::SPF => {
                        Ok(Some(ParseResult::PMTK(PMTKSPF::parse(source, &mut iter)?)))
                    }
                }
            }
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
fn test_source_bitor() {
    let s = Source::GLONASS | Source::GPS | Source::Beidou;
    assert!(s.mask == (Source::GLONASS as u32 | Source::GPS as u32 | Source::Beidou as u32));
}

#[test]
fn test_sentence_bitor() {
    let s = Sentence::RMC | Sentence::VTG | Sentence::GGA;
    assert!(s.mask == (Sentence::RMC as u32 | Sentence::VTG as u32 | Sentence::GGA as u32));
}

#[test]
fn test_create_filtered_parser() {
    let _parser = Parser::new()
        .source_filter(Source::GPS | Source::GLONASS)
        .sentence_filter(Sentence::RMC | Sentence::GLL);
    let _parser = Parser::new()
        .source_only(Source::GPS)
        .sentence_only(Sentence::RMC);
}
