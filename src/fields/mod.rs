use super::separators::Separators;
use super::*;

/// Represents a single field inside the HL7.  Note that fields can include repeats, components and sub-components.
/// See [the spec](http://www.hl7.eu/HL7v2x/v251/std251/ch02.html#Heading13) for more info
#[derive(Debug, PartialEq, Clone, Copy)]

pub struct Field<'a> {
    pub source: &'a str,
    pub delims: (char, char),
}

impl<'a> Field<'a> {
    /// Convert the given line of text into a field.
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Field<'a>, Hl7ParseError> {
        let field = Field {
            source: &input,
            delims: (delims.component, delims.subcomponent)
        };
        Ok(field)
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
        self.source
    }

    /// Export valus to owned String
    pub fn to_string(&self) -> String {
        self.source.clone().to_owned()
    }

    /// Export valus to str
    pub fn as_str(&self) -> &'a str {
        self.source
    }

    /// Method to get the underlying components of the value in this field.
    pub fn components(&self) -> Vec<&'a str> {
        self.source.split(self.delims.0).collect()
    }

    /// Method to get the subcomponents from the value in this field.
    pub fn subcomponents(&self) -> Vec<Vec<&'a str>> {
        let components = self.source
            .split(self.delims.0).collect::<Vec<&'a str>>();
        components
            .iter()
            .map(|sc| sc.split(self.delims.1).collect::<Vec<&'a str>>())
            .collect()
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
        assert_eq!(f.components().len(), 2)
    }

    #[test]
    fn test_parse_subcomponents() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.subcomponents()[1].len(), 2)
    }

    #[test]
    fn test_to_string() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.to_string(), String::from("xxx^yyy&zzz"))
    }
}
