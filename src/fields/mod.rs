use super::separators::Separators;
use super::*;
use std::ops::Index;

/// Represents a single field inside the HL7.  Note that fields can include repeats, components and sub-components.
/// See [the spec](http://www.hl7.eu/HL7v2x/v251/std251/ch02.html#Heading13) for more info
#[derive(Debug, PartialEq)]

pub struct Field<'a> {
    pub source: &'a str,
    pub delims: Separators,
    pub components: Vec<&'a str>,
    pub subcomponents: Vec<Vec<&'a str>>,
}

impl<'a> Field<'a> {
    /// Convert the given line of text into a field.
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Field<'a>, Hl7ParseError> {
        let components = input.split(delims.component).collect::<Vec<&'a str>>();
        let subcomponents = components
            .iter()
            .map(|c| c.split(delims.subcomponent).collect::<Vec<&'a str>>())
            .collect();
        let field = Field {
            source: &input,
            delims: *delims,
            components,
            subcomponents,
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

    /// Compatibility method to get the underlying value of this field.
    pub fn value(&self) -> &'a str {
        self.source
    }

    /// Export value to owned String
    pub fn to_string(&self) -> String {
        self.source.clone().to_owned()
    }

    /// Export value to str
    pub fn as_str(&self) -> &'a str {
        self.source
    }
}

impl<'a> Clone for Field<'a> {
    /// Creates a new Message object using a clone of the original's source
    fn clone(&self) -> Self {
        Field::parse(self.source, &self.delims.clone()).unwrap()
    }
}

/// Access string reference of a Field component by numeric index
impl<'a> Index<usize> for Field<'a> {
    type Output = &'a str;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.components[idx]
    }
}

/// Access string reference of a Field subcomponent by numeric index
impl<'a> Index<(usize, usize)> for Field<'a> {
    type Output = &'a str;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.subcomponents[idx.0][idx.1]
    }
}

/// Access string reference of a Field component by String index
/// Adjust the index by one as medical people do not count from zero
impl<'a> Index<String> for Field<'a> {
    type Output = &'a str;
    fn index(&self, sidx: String) -> &Self::Output {
        let parts = sidx.split('.').collect::<Vec<&str>>();
        if parts.len() == 1 {
            let stringnums = parts[0].chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>();
            let idx: usize = stringnums.parse().unwrap();
            return &self[idx - 1]
        } else if parts.len() == 2 {
            let stringnums = parts[0].chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>();
            let idx0: usize = stringnums.parse().unwrap();
            let stringnums = parts[1].chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>();
            let idx1: usize = stringnums.parse().unwrap();
            &self[(idx0 - 1,idx1 - 1)]
        } else {
            &""
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
        assert_eq!(f.components.len(), 2)
    }

    #[test]
    fn test_parse_subcomponents() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.subcomponents[1].len(), 2)
    }

    #[test]
    fn test_to_string() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.to_string(), String::from("xxx^yyy&zzz"))
    }

    #[test]
    fn test_clone() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f.to_string(), f.clone().as_str())
    }

    #[test]
    fn test_uint_index() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        assert_eq!(f[1], "yyy&zzz");
        assert_eq!(f[(1, 1)], "zzz");
    }

    #[test]
    fn test_string_index() {
        let d = Separators::default();
        let f = Field::parse_mandatory(Some("xxx^yyy&zzz"), &d).unwrap();
        let idx0 = String::from("R2");
        let idx1 = String::from("R2.C2");
        assert_eq!(f[idx0.clone()], "yyy&zzz");
        assert_eq!(f[idx1], "zzz");
    }
}
