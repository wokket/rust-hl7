/*!
    This module contains the decoding functionality to parse escape sequences from the source string back to their original chars.

    For more info see [here](https://www.lyniate.com/knowledge-hub/hl7-escape-sequences/) or [here](https://confluence.hl7australia.com/display/OOADRM20181/Appendix+1+Parsing+HL7v2#Appendix1ParsingHL7v2-Dealingwithreservedcharactersanddelimiters)


*/

use std::borrow::Cow;
use regex::{Match, Regex};
use crate::separators::Separators;


pub struct EscapeSequence {
    delims: Separators,
    regex: Regex
}

impl<'a> EscapeSequence {

    pub fn new(delims: Separators) -> EscapeSequence {
        EscapeSequence { 
            delims,
            regex: Regex::new("[/]").unwrap()
        }
    }


    pub fn decode<S>(&self, input: S) -> Cow<'a, str> 
        where S: Into<Cow<'a, str>>
    {
        
        let input = input.into();
        let first = self.regex.find(&input); // find the first escape sequence

        if first.is_some() {
            input.into()
        } else { // no escape char in the string at all, just return what we have
            input.into()
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_does_nothing_if_not_required() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);


        let input = "There are no escape sequences here/there.";
        let output = escaper.decode(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_decode_handles_field_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);


        let input = "Escape this \\F\\ please";
        let output = escaper.decode(input);
        assert_eq!(output, "Escape this | please");
    }


}