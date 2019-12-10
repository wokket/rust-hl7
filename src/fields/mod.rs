use super::separators::Separators;
use super::*;

/// Represents a single field inside the HL7.  Note that fields can include repeats, components and sub-components.
/// See [the spec](http://www.hl7.eu/HL7v2x/v251/std251/ch02.html#Heading13) for more info
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Field<'a> {
    Generic(&'a str),
}

impl<'a> Field<'a> {
    /// Convert the given line of text into a field.
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Field<'a>, Hl7ParseError> {
        Ok(Field::Generic(input))
    }

    /// Converts a possible blank string into a possible blank field!  
    /// Note this handles optional fields, not the nul (`""`) value.
    pub fn parse_optional(
        input: Option<&'a str>,
        delims: &Separators,
    ) -> Result<Option<Field<'a>>, Hl7ParseError> {
        match input {
            None => Ok(None),
            Some(x) => Ok(Some(Field::parse(x, delims)?)),
        }
    }
}

// We currently use this conversion for matching Segment[0] to determine what type Segment to return.
impl<'a> Into<&'a str> for Field<'a> {
    fn into(self) -> &'a str {
        match self {
            Field::Generic(s) => s,
        }
    }
}
