extern crate test;

mod field_parser;
pub mod message_parser;
mod segment_parser;

/// A repeat of a field is a set of 0 or more sub component values.
/// Currently all values are stored as their original string representations.  Methods to convert
/// the values to their HL7-spec types is outside the scope of the parser.
#[derive(Debug, PartialEq)]
pub struct Repeat<'a> {
    pub sub_components: Vec<&'a str>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub repeats: Vec<Repeat<'a>>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
pub struct Segment<'a> {
    pub fields: Vec<Field<'a>>,
}

/// A Message is an entire HL7 message parsed into it's consitituent segments, fields, repeats and subcomponents
/// It consists of (1 or more) Segments.
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub segments: Vec<Segment<'a>>,
}
