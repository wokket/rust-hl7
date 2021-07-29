use super::segments::*;
use super::separators::Separators;
use super::*;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponents,
/// and it consists of (1 or more) Segments.
/// Message parses the source string into &str slices (minimising copying)
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub source: &'a str,
    pub segments: Vec<Segment<'a>>,
}

impl<'a> Message<'a> {
    /// Takes the source HL7 string and parses it into this message.  Segments
    /// and other data are slices (`&str`) into the source HL7
    pub fn from_str(source: &'a str) -> Result<Self, Hl7ParseError> {
        let delimiters = str::parse::<Separators>(source)?;

        let segments: Result<Vec<Segment<'a>>, Hl7ParseError> = source
            .split(delimiters.segment)
            .map(|line| Segment::parse(line, &delimiters))
            .collect();

        let msg = Message {
            source,
            segments: segments?,
        };

        Ok(msg)
    }

    /// Extracts header element to an owned object for external use
    pub fn msh(&self) -> Result<msh::MshSegment, Hl7ParseError> {
        let segment = self
            .segments
            .iter()
            .find_map(|s| match s {
                segments::Segment::MSH(x) => Some(x.clone().to_owned()),
                _ => None,
            })
            .expect("Failed to find hl7 header");
        Ok(segment)
    }

    /// Extracts all generic elements to owned objects for external use
    pub fn generic_segments(&self) -> Result<Vec<&generic::GenericSegment>, Hl7ParseError> {
        let generics: Vec<&generic::GenericSegment> = self
            .segments
            .iter()
            .filter_map(|s| match s {
                segments::Segment::Generic(x) => Some(x),
                _ => None,
            })
            .map(|g| g.clone().to_owned())
            .collect();
        Ok(generics)
    }

    /// Extracts generic elements to owned objects for external use by matching first field to name
    pub fn generic_segments_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<&generic::GenericSegment>, Hl7ParseError> {
        let found: Vec<&generic::GenericSegment> = self
            .segments
            .iter()
            .filter_map(|s| match s {
                segments::Segment::Generic(x) => {
                    if x.fields.first().unwrap().value() == name {
                        Some(x)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map(|g| g.clone().to_owned())
            .collect();
        Ok(found)
    }

    /// Present input vectors of &generics to vectors of &str
    pub fn segs_to_str_vecs(
        segments: Vec<&generic::GenericSegment<'a>>,
    ) -> Result<Vec<Vec<&'a str>>, Hl7ParseError> {
        let vecs = segments
            .iter()
            .map(|s| s.fields.iter().map(|f| f.value()).collect())
            .collect();
        Ok(vecs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_segments_are_added() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.segments.len(), 2);
        Ok(())
    }

    #[test]
    fn ensure_generic_segments_are_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.generic_segments().unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn ensure_generic_segments_are_found() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.generic_segments_by_name("OBR").unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn ensure_msh_is_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.msh().unwrap().msh_1_field_separator, '|');
        Ok(())
    }
    #[test]
    fn ensure_segments_convert_to_vectors() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;
        let segs = msg.generic_segments_by_name("OBR")?;
        let sval = segs.first().unwrap().fields.first().unwrap().value();
        let vecs = Message::segs_to_str_vecs(segs).unwrap();
        let vval = vecs.first().unwrap().first().unwrap();

        assert_eq!(vval, &sval);
        Ok(())
    }
}
