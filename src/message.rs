use super::segments::*;
use super::separators::Separators;
use super::*;
use std::str::FromStr;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponent
/// It consists of (1 or more) Segments.
#[derive(Debug, PartialEq)]
pub struct Message {
    segments: Vec<Segment>,
}

impl FromStr for Message {
    type Err = Hl7ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let delimiters = str::parse::<Separators>(input)?;
        let mut msg = Message {
            segments: Vec::new(),
        };

        for line in input.split(delimiters.segment) {
            let seg = Segment::parse(line, &delimiters)?;
            msg.segments.push(seg);
        }

        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_segments_are_added() {
        let hl7 = "MSH|fields\rOBR|segment";
        let msg = match str::parse::<Message>(hl7) {
            Ok(x) => x,
            Err(e) => {
                assert!(false, e);
                return;
            }
        };

        assert_eq!(msg.segments.len(), 2);
    }
}
