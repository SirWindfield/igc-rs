#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::util::ParseError;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "lowercase"))]
pub enum GpsQualifier {
    Gps,
    DGps,
}

/// Differential GPS record - indicates that Differential GPS is being used.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DRecord<'a> {
    pub qualifier: GpsQualifier,
    pub station_id: &'a str,
}

impl<'a> DRecord<'a> {
    pub fn parse(line: &'a str) -> Result<Self, ParseError> {
        if line.len() != 6 {
            return Err(ParseError::SyntaxError);
        }

        let bytes = line.as_bytes();
        assert_eq!(bytes[0], b'D');

        let qualifier = match bytes[1] {
            b'1' => GpsQualifier::Gps,
            b'2' => GpsQualifier::DGps,
            _ => return Err(ParseError::SyntaxError),
        };

        let station_id = &line[2..6];

        Ok(DRecord {
            qualifier,
            station_id,
        })
    }
}

impl<'a> fmt::Display for DRecord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let qual_str = match self.qualifier {
            GpsQualifier::Gps => '1',
            GpsQualifier::DGps => '2',
        };

        write!(f, "D{}{}", qual_str, self.station_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{DRecord, GpsQualifier};

    #[test]
    fn drecord_parse() {
        let example_line = "D1ABCD";
        let parsed = DRecord::parse(example_line).unwrap();
        let expected = DRecord {
            qualifier: GpsQualifier::Gps,
            station_id: "ABCD",
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn drecord_format() {
        let expected_string = "D1ABCD";
        let record = DRecord {
            qualifier: GpsQualifier::Gps,
            station_id: "ABCD",
        };

        assert_eq!(format!("{}", record), expected_string);
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn parse_doesnt_crash(s in "D\\PC*") {
            DRecord::parse(&s);
        }
    }
}
