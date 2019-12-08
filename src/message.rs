use super::segments::*;
use super::separators::Separators;
use super::*;
use std::str::FromStr;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponent
/// It consists of (1 or more) Segments.
#[derive(Debug, PartialEq)]
pub struct Message {
    Segments: Vec<Segment>,
}

impl FromStr for Message {
    type Err = Hl7ParseError;

    fn FromStr(input: &str) -> Result<Self, Self::Err> {
        let delimiters = String::parse::<Separators>(input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_separators_load_correctly() {}
}
