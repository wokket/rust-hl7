use super::fields::Field;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq)]
pub struct GenericSegment<'a> {
    pub source: &'a str,
    pub delim: char,
    pub fields: Vec<Field<'a>>,
}

impl<'a> GenericSegment<'a> {
    /// Export source to owned String
    pub fn to_string(&self) -> String {
        self.source.clone().to_owned()
    }

    /// Export source to str
    pub fn as_str(&self) -> &'a str {
        self.source
    }
}
