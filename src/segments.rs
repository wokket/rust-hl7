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
