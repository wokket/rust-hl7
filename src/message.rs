use super::segments::*;
use super::separators::Separators;
use super::*;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponents,
/// and it consists of (1 or more) Segments.
/// Message parses the source string into &str slices (minimising copying)
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    source: &'a str,
    segments: Vec<Segment<'a>>,
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
            source: source,
            segments: segments?,
        };
        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_segments_are_added() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|fields\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.segments.len(), 2);
        Ok(())
    }
}
