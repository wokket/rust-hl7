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
        let segment = self.segments.iter()
            .find_map(|s| match s {
                segments::Segment::MSH(x) => Some(x.clone().to_owned()),
                _ => None,
            })
            .expect("Failed to find hl7 header");
        Ok(segment)
    }

    /// Extracts all generic elements to owned objects for external use
    pub fn generics(&self) -> Result<Vec<&generic::GenericSegment>, Hl7ParseError> {
        let generics: Vec<&generic::GenericSegment> = self.segments.iter()
            .filter_map(|s| match s {
                segments::Segment::Generic(x) => Some(x),
                _ => None,
            }).map(|g| g.clone().to_owned()).collect();
        Ok(generics)
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
}
