[![Crates.io](https://img.shields.io/badge/crates.io-v0.3.0-orange.svg?longCache=true)](https://crates.io/crates/nmea0183)
[![Build Status](https://travis-ci.org/nsforth/nmea0183.svg?tag=v0.3.0)](https://travis-ci.org/nsforth/nmea0183)
[![Codecov coverage status](https://codecov.io/gh/nsforth/nmea0183/branch/master/graph/badge.svg)](https://codecov.io/gh/nsforth/nmea0183)
# NMEA 0183 parser.

Implemented most used sentences like RMC, VTG, GGA, GLL.
Parser do not use heap memory and relies only on `core`.

You should instantiate [Parser](https://docs.rs/nmea0183/latest/nmea0183/struct.Parser.html) with [new](https://docs.rs/nmea0183/latest/nmea0183/struct.Parser.html#method.new) and than use methods like [parse_from_byte](https://docs.rs/nmea0183/latest/nmea0183/struct.Parser.html#method.parse_from_bytes) or [parse_from_bytes](https://docs.rs/nmea0183/latest/nmea0183/struct.Parser.html#method.parse_from_bytes).
If parser accumulates enough data it will return [ParseResult](https://docs.rs/nmea0183/latest/nmea0183/enum.ParseResult.html) on success or `&str` that describing an error.

You do not need to do any preprocessing such as split data to strings or NMEA sentences.

# Examples

If you could read a one byte at a time from the receiver you may use `parse_from_byte`:
```rust
use nmea0183::{Parser, ParseResult};

let nmea = b"$GPGGA,145659.00,5956.695396,N,03022.454999,E,2,07,0.6,9.0,M,18.0,M,,*62\r\n$GPGGA,,,,,,,,,,,,,,*00\r\n";
let mut parser = Parser::new();
for b in &nmea[..] {
    if let Some(result) = parser.parse_from_byte(*b) {
        match result {
            Ok(ParseResult::GGA(Some(gga))) => { }, // Got GGA sentence
            Ok(ParseResult::GGA(None)) => { }, // Got GGA sentence without valid data, receiver ok but has no solution
            Ok(_) => {}, // Some other sentences..
            Err(e) => { } // Got parse error
        }
    }
}
```

If you read many bytes from receiver at once or want to parse NMEA log from text file you could use Iterator-style:
```rust
use nmea0183::{Parser, ParseResult};

let nmea = b"$GPGGA,,,,,,,,,,,,,,*00\r\n$GPRMC,125504.049,A,5542.2389,N,03741.6063,E,0.06,25.82,200906,,,A*56\r\n";
let mut parser = Parser::new();

for result in parser.parse_from_bytes(&nmea[..]) {
    match result {
        Ok(ParseResult::RMC(Some(rmc))) => { }, // Got RMC sentence
        Ok(ParseResult::GGA(None)) => { }, // Got GGA sentence without valid data, receiver ok but has no solution
        Ok(_) => {}, // Some other sentences..
        Err(e) => { } // Got parse error
    }
}
```

It is possible to ignore some sentences or sources. You can set filter on [Parser](https://docs.rs/nmea0183/latest/nmea0183/struct.Parser.html) like so:
```rust
use nmea0183::{Parser, ParseResult, Sentence, Source};

let parser_only_gps_gallileo = Parser::new()
    .source_filter(Source::GPS | Source::Gallileo);
let parser_only_rmc_gga_gps = Parser::new()
    .source_only(Source::GPS)
    .sentence_filter(Sentence::RMC | Sentence::GGA);
```

# Panics

Should not panic. If so please report issue on project page.

# Errors

`Unsupported sentence type.` - Got currently not supported sentence.

`Checksum error!` - Sentence has wrong checksum, possible data corruption.

`Source is not supported!` - Unknown source, new sattelite system is launched? :)

`NMEA format error!` - Possible data corruption. Parser drops all accumulated data and starts seek new sentences.

It's possible to got other very rare error messages that relates to protocol errors. Receivers nowadays mostly do not violate NMEA specs.

# Planned features

GSA and GSV parsing.

