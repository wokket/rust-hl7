use super::fields::Field;
use std::ops::Index;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq, Clone)]
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

impl<'a> Index<usize> for GenericSegment<'a> {
    type Output = &'a str;
    fn index(&self, fidx: usize) -> &Self::Output {
        &self.fields[fidx].source
    }
}
