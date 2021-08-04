use super::fields::Field;
use super::separators::Separators;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq)]
pub struct GenericSegment<'a> {
    pub fields: Vec<Field<'a>>,
}

impl<'a> GenericSegment<'a> {
    pub fn to_string(&self, delims: &Separators) -> String {
        let field = String::from(delims.field);
        let fields = self.fields[..]
            .iter()
            .map(|f| f.value())
            .collect::<Vec<&'a str>>();
        fields.join(&field)
    }
}
