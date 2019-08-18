use core::convert::TryFrom;
use nmea_0183::coords;
use nmea_0183::datetime;
use nmea_0183::gga;
use nmea_0183::modes;
use nmea_0183::rmc;
use nmea_0183::gll;
use nmea_0183::vtg;
use nmea_0183::{ParseResult, Parser, Source};

#[test]
fn test_correct_but_unsupported_source() {
    let mut p = Parser::new();
    let sentence = b"$LCVTG,089.0,T,,,15.2,N,,*67\r\n";
    let mut parsed = false;
    for b in sentence.iter() {
        let r = p.parse_from_byte(*b);
        if r.is_some() {
            assert_eq!(r.unwrap(), Err("Source is not supported"));
            parsed = true;
            break;
        }
    }
    if !parsed {
        panic!("Parser failed to parse correct block!");
    }
}

#[test]
fn test_correct_but_unsupported_nmea_block() {
    let mut p = Parser::new();
    let sentence = b"$GPZZZ,,,,,,,,,*61\r\n";
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
fn test_correct_vtg() {
    let mut p = Parser::new();
    let sentence = b"$GPVTG,089.0,T,,,15.2,N,,,A*12\r\n";
    let mut parsed = false;
    for b in sentence.iter() {
        let r = p.parse_from_byte(*b);
        if r.is_some() {
            assert_eq!(r.unwrap(), Ok(ParseResult::VTG(Some(vtg::VTG {
                source: Source::GPS,
                course: From::from(89.0),
                magnetic: None,
                speed: coords::Speed::from_knots(15.2),
                mode: modes::Mode::Autonomous
            }))));
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
                    course: From::from(25.82),
                    magnetic: None,
                    mode: modes::Mode::Autonomous
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
fn test_correct_gga() {
    let mut p = Parser::new();
    let sentence = b"$GPGGA,145659.00,5956.695396,N,03022.454999,E,2,07,0.6,9.0,M,18.0,M,,*62\r\n";
    let mut parsed = false;
    for b in sentence.iter() {
        let r = p.parse_from_byte(*b);
        if r.is_some() {
            assert_eq!(
                r.unwrap(),
                Ok(ParseResult::GGA(Some(gga::GGA {
                    source: Source::GPS,
                    time: datetime::Time {
                        hours: 14,
                        minutes: 56,
                        seconds: 59.0
                    },
                    latitude: TryFrom::try_from(59.944923266667).unwrap(),
                    longitude: TryFrom::try_from(30.3742499833).unwrap(),
                    gps_quality: gga::GPSQuality::DGPS,
                    sat_in_use: 7,
                    hdop: 0.6,
                    altitude: coords::Altitude { meters: 9.0 },
                    geoidal_separation: Some(18.0),
                    age_dgps: None,
                    dgps_station_id: None
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
                    course: From::from(255.6),
                    magnetic: Some(From::from(246.90001)),
                    mode: modes::Mode::Autonomous
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
                course: From::from(25.82),
                magnetic: None,
                mode: modes::Mode::Autonomous
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
                course: From::from(25.82),
                magnetic: None,
                mode: modes::Mode::Autonomous
            })))
        );
        assert!(iter.next().is_none());
    }
}
