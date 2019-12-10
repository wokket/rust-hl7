pub mod generic;
pub mod msh;

use super::fields::Field;
use super::separators::Separators;
use super::*;
use generic::GenericSegment;
use msh::MshSegment;

/// A single segment, 0x13 delimited line from a source HL7 message consisting of multiple fields.
#[derive(Debug, PartialEq)]
pub enum Segment<'a> {
    MSH(MshSegment<'a>),
    Generic(GenericSegment<'a>),
}

impl<'a> Segment<'a> {
    /// Convert the given line of text into a Segment.
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Segment<'a>, Hl7ParseError> {
        let fields: Result<Vec<Field<'a>>, Hl7ParseError> = input
            .split(delims.field)
            .map(|line| Field::parse(line, &delims))
            .collect();

        let fields = fields?;

        let seg = match fields[0].into() {
            "MSH" => Segment::MSH(MshSegment { fields }),
            _ => Segment::Generic(GenericSegment { fields }),
        };

        Ok(seg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_fields_are_added() -> Result<(), Hl7ParseError> {
        let hl7 = "SEG|field 1|field 2|field 3";
        let delims = Separators::default();

        if let Segment::Generic(seg) = Segment::parse(hl7, &delims)? {
            assert_eq!(seg.fields.len(), 4);
        } else {
            assert!(false);
        }
        Ok(())
    }

    #[test]
    fn ensure_msh_is_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|field 1|field 2|field 3";
        let delims = Separators::default();

        if let Segment::MSH(seg) = Segment::parse(hl7, &delims)? {
            assert_eq!(seg.fields.len(), 4);
        } else {
            assert!(false);
        }
        Ok(())
    }
}
