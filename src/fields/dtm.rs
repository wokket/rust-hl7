//NOTE: This class is a WIP, currently stymied
// about how to represent the various formats permitted by a HL7 DTM field,
// while using strictly-correct libraries like chrono.
// "If it involves date and times, it's buggy."

use super::Hl7ParseError;
use chrono::format::ParseError;
use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, Utc};

/// Wraps a string into a DateTime, and provides access to it.
//#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DTMField<'a> {
    input: &'a str,
    value: Datelike,
}

impl<'a> DTMField<'a> {
    /// Method to get the underlying value of this field.
    pub fn value(&self) -> &'a str {
        self.input
    }

    pub fn parse(input: &'a str) -> Result<Self, Hl7ParseError> {
        let date_time = NaiveDateTime::parse_from_str(input, "%Y%m%d")?;

        let field = DTMField {
            input,
            value: date_time,
        };

        Ok(field)
    }
}

impl From<ParseError> for Hl7ParseError {
    fn from(error: ParseError) -> Self {
        Hl7ParseError::InvalidDateTimeFormat { error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_from_chrono_error_to_hl7_parse_error() {
        match NaiveDateTime::parse_from_str("20141338", "%Y%m%d") {
            //out of range
            Err(parse_error) => {
                let hl7: Hl7ParseError = parse_error.into();
                assert_eq!(
                    hl7,
                    Hl7ParseError::InvalidDateTimeFormat { error: parse_error }
                );
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_year_only_input() {
        match DTMField::parse("20200102") {
            Ok(field) => {
                let output = field.value.format("%Y %m %d");
                assert_eq!(output.to_string(), "2020 01 02");
            }
            Err(err) => {
                println!("Fail: {}", err);
                assert!(false, err);
            }
        }
    }
}
