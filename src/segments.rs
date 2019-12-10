use super::separators::Separators;
use super::*;

type Field = String; //temp till we write it

/// A single segment, 0x13 delimited line from a source HL7 message consisting of multiple fields.
#[derive(Debug, PartialEq)]
pub enum Segment<'a> {
    MSH(MshSegment<'a>),
    Generic(GenericSegment<'a>),
}

#[derive(Debug, PartialEq)]
pub struct MshSegment<'a> {
    fields: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct GenericSegment<'a> {
    fields: Vec<&'a str>,
}

impl<'a> Segment<'a> {
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Segment<'a>, Hl7ParseError> {
        let fields: Vec<&'a str> = input.split(delims.field).collect();

        let seg = match fields[0] {
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
