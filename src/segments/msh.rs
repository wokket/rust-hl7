use super::fields::Field;

/// The most important Segment, almost all HL7 messages have an MSH (MLLP simple ack I'm looking at you).
/// Given the importance of this segment for driving application behaviour, it gets the special treatment
/// of a fully typed segment, not just a bag of fields....
#[derive(Debug, PartialEq)]
pub struct MshSegment<'a> {
    pub fields: Vec<Field<'a>>,
}
