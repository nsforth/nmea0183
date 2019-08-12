//#![no_std]

enum Sentences {
    RMC,
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

    pub fn parse(&mut self, input: &[u8]) -> Result<u8, &'static str> {
        for b in input {
            let symbol = *b;
            match self.parser_state {
                ParserState::WaitStart if symbol == b'$' => {
                    self.parser_state = ParserState::ReadUntilChkSum;
                    eprintln!("Transition to: {:?}", self.parser_state);
                    self.buflen = 0;
                    self.chksum = 0;
                }
                ParserState::WaitStart if symbol != b'$' => continue,
                ParserState::ReadUntilChkSum if symbol != b'*' => {
                    if self.buffer.len() < self.buflen {
                        return Err("NMEA sentence is too long!");
                    } else {
                        self.buffer[self.buflen] = symbol;
                        self.buflen += 1;
                        self.chksum = self.chksum ^ symbol;
                    }
                }
                ParserState::ReadUntilChkSum if symbol == b'*' => {
                    self.parser_state = ParserState::ChkSumUpper;
                    eprintln!("Transition to: {:?}", self.parser_state);
                }
                ParserState::ChkSumUpper => {
                    self.expected_chksum = parse_hex_halfbyte(symbol)?;
                    self.parser_state = ParserState::ChkSumLower;
                    eprintln!("Transition to: {:?}", self.parser_state);
                }
                ParserState::ChkSumLower => {
                    self.expected_chksum =
                        (self.expected_chksum << 4) | parse_hex_halfbyte(symbol)?;
                    if self.expected_chksum != self.chksum {
                        self.parser_state = ParserState::WaitStart;
                        return Err("Checksum error!");
                    } else {
                        self.parser_state = ParserState::WaitCR;
                        eprintln!("Transition to: {:?}", self.parser_state);
                    }
                }
                ParserState::WaitCR if symbol == b'\r' => {
                    self.parser_state = ParserState::WaitLF;
                    eprintln!("Transition to: {:?}", self.parser_state);
                }
                ParserState::WaitLF if symbol == b'\n' => {
                    self.parser_state = ParserState::WaitStart;
                    return Ok(1);
                    // TODO Sentences processing
                }
                _ => {
                    self.parser_state = ParserState::WaitStart;
                    return Err("NMEA format error!");
                }
            }
        }
        Ok(0)
    }
}

fn parse_hex_halfbyte(symbol: u8) -> Result<u8, &'static str> {
    if symbol >= b'0' && symbol <= b'9' {
        return Ok(symbol - b'0');
    }
    if symbol >= b'A' && symbol <= b'F' {
        return Ok(symbol - b'A' + 10);
    }
    Err("Invalid character. Should be one of 0123456789ABCDEF")
}

#[test]
fn test_correct_nmea_block() {
    let mut p = Parser::new();
    let b = b"$GPVTG,089.0,T,,,15.2,N,,*7F\r\n";
    assert_eq!(p.parse(&b[..]), Ok(1));
}
