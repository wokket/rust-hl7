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
    pub fn parse(input: &'a str, _delims: &Separators) -> Result<Field<'a>, Hl7ParseError> {
        // TODO: Match on component delim and return one of two Field variants (Generic or Componentised??)... names are hard
        Ok(Field::Generic(input))
    }

    /// Used to hide the removal of NoneError for #2...  If passed `Some()` value it retursn a field with that value.  If passed `None() it returns an `Err(Hl7ParseError::MissingRequiredValue{})`
    pub fn parse_mandatory(
        input: Option<&'a str>,
        delims: &Separators,
    ) -> Result<Field<'a>, Hl7ParseError> {
        match input {
            Some(string_value) => Field::parse(string_value, delims),
            None => Err(Hl7ParseError::MissingRequiredValue {}),
        }
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
        }
    }

    /// Method to get the underlying components of the value in this field.
    pub fn components(&self, delims: &Separators) -> Vec<&'a str> {
        match self {
            Field::Generic(s) => s.split(delims.component).collect(),
        }
    }

    /// Method to get the subcomponents from the value in this field.
    pub fn subcomponents(&self, delims: &Separators) -> Vec<Vec<&'a str>> {
        match self {
            Field::Generic(s) => {
                let components = s.split(delims.component).collect::<Vec<&'a str>>();
                components
                    .iter()
                    .map(|sc| sc.split(delims.subcomponent).collect::<Vec<&'a str>>())
                    .collect()
            }
        }
    }

    /// Extract field value to owned String
    pub fn to_string(&self) -> String {
        match self {
            Field::Generic(s) => s.clone().to_owned()
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

    #[test]
    fn test_parse_mandatory_handles_some_value() {
        let d = Separators::default();

        match Field::parse_mandatory(Some("xxx"), &d) {
            Ok(field) => assert_eq!(field.value(), "xxx"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_mandatory_throws_on_none() {
        let d = Separators::default();

        match Field::parse_mandatory(None, &d) {
            Err(Hl7ParseError::MissingRequiredValue()) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_components() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy"), &d).unwrap();
        assert_eq!(f.components(&d).len(), 2)
    }

    #[test]
    fn test_parse_subcomponents() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.subcomponents(&d)[1].len(), 2)
    }

    #[test]
    fn test_to_string() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.to_string(), String::from("xxx^yyy&zzz"))
    }
}
