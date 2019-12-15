use super::separators::Separators;
use super::*;
use fields::dtm::DTMField;

pub mod dtm;

/// Represents a single field inside the HL7.  Note that fields can include repeats, components and sub-components.
/// See [the spec](http://www.hl7.eu/HL7v2x/v251/std251/ch02.html#Heading13) for more info
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Field<'a> {
    /// A generic field that just holds a string.  Simple to use, nothing special in return.
    Generic(&'a str),
    /// A HL7 DTM (Date Time field.  See [the spec](http://www.hl7.eu/refactored/dtDTM.html) for more info)
    DateTime(DTMField<'a>)
}

impl<'a> Field<'a> {
    /// Convert the given line of text into a field.
    pub fn parse(input: &'a str, _delims: &Separators) -> Result<Field<'a>, Hl7ParseError> {
        Ok(Field::Generic(input))
        //todo: repeats, field types etc
    }

    /// parses the given slice into a DTM (datetime) field 
    pub fn parse_datetime(input: &'a str) -> Result<Field<'a>, Hl7ParseError>{
       let info =  DTMField::parse(input)?;
       Ok(Field::DateTime(info))
    }

    /// Converts a possibly blank string into a possibly blank field!  
    /// Note this handles optional fields, not the nul (`""`) value.
    pub fn parse_optional(
        input: Option<&'a str>,
        delims: &Separators,
    ) -> Result<Option<Field<'a>>, Hl7ParseError> {
        match input {
            None => Ok(None),
            Some(x) if x.len() == 0 => Ok(None),
            Some(x) => Ok(Some(Field::parse(x, delims)?)),
        }
    }

    /// Method to get the underlying value of this field.
    /// If this is a GenericField this method does not allocate.
    pub fn value(&self) -> &'a str {
        match self {
            Field::Generic(s) => s,
            Field::DateTime(f) => f.value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_parse_handles_none() {
        let d = Separators::default();

        //if we pass a none value, we get a None back
        match Field::parse_optional(None, &d) {
            Ok(None) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_conditional_parse_handles_empty_string() {
        let d = Separators::default();

        //an empty string (as seen when `split()`ing) should be none
        match Field::parse_optional(Some(""), &d) {
            Ok(None) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_conditional_parse_handles_value_string() {
        let d = Separators::default();

        //an empty string (as seen when `split()`ing) should be none
        match Field::parse_optional(Some("xxx"), &d) {
            Ok(Some(field)) => assert_eq!(field.value(), "xxx"),
            _ => assert!(false),
        }
    }
}
