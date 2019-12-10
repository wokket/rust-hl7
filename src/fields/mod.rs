use super::separators::Separators;
use super::*;

/// Represents a single field inside the HL7.  Note that fields can include repeats, components and sub-components
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Field<'a> {
    Generic(&'a str),
}

impl<'a> Field<'a> {
    /// Convert the given line of text into a field.
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Field<'a>, Hl7ParseError> {
        Ok(Field::Generic(input))
    }
}

impl<'a> Into<&'a str> for Field<'a> {
    fn into(self) -> &'a str {
        match self {
            Field::Generic(s) => s,
        }
    }
}
