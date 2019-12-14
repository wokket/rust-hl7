use super::fields::Field;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq)]
pub struct GenericSegment<'a> {
    pub fields: Vec<Field<'a>>,
}
