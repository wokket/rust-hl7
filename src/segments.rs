use super::separators::Separators;
use super::*;

/// A single segment, 0x13 delimited line from a source HL7 message consisting of multiple fields.
#[derive(Debug, PartialEq)]
pub enum Segment {
    MSH(MshSegment),
    Generic(GenericSegment),
}

#[derive(Debug, PartialEq)]
pub struct MshSegment;

#[derive(Debug, PartialEq)]
pub struct GenericSegment;

impl Segment {
    pub fn parse(input: &str, delims: &Separators) -> Result<Segment, Hl7ParseError> {
        Ok(Segment::Generic(GenericSegment {}))
    }
}
