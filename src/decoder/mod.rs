/*!
    This module contains the decoding functionality to parse escape sequences from the source string back to their original chars.

    For more info see [here](https://www.lyniate.com/knowledge-hub/hl7-escape-sequences/) or [here](https://confluence.hl7australia.com/display/OOADRM20181/Appendix+1+Parsing+HL7v2#Appendix1ParsingHL7v2-Dealingwithreservedcharactersanddelimiters)

    ## Details

    This decoder will replace some, ** but not all ** of the standard HL7 escape sequences.  Specifically, the following sequences are **NOT** replaced:
    - `\H\` - Indicates the start of highlighted text, this is a consuming application problem and will not be replaced
    - `\N\` - Indicates the end of highlighted text and resumption of normal text.  This is a consuming application problem and will not be replaced
    - `\Z...\` - Custom application escape sequences, these are custom (as are most `Z` items in HL7) and will not be replaced
*/

use crate::separators::Separators;
use regex::Regex;
use std::borrow::Cow;

pub struct EscapeSequence {
    delims: Separators,
    escape_char_regex: Regex,
    field_regex: Regex,
}

impl<'a> EscapeSequence {
    pub fn new(delims: Separators) -> EscapeSequence {
        let return_val = EscapeSequence {
            delims,
            escape_char_regex: Regex::new(r#"\\"#).unwrap(),
            field_regex: Regex::new(r#"\\F\\"#).unwrap(),
        };

        return_val
    }

    pub fn decode<S>(&self, input: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        let input = input.into();
        //let first = input.find(self.delims.escape_char);
        let first = self.escape_char_regex.find(&input);

        if first.is_some() {
            // We know there's a backslash, so we need to process stuff
            // I wanted to use regex as a simple(ish) if slow(ish) way to get started with this, but the requirement for a dynamic escaping char (typically `\`)
            // which may/may not need doubling up for regex interpretation reasons is making that harder...

            let output = self
                .field_regex
                .replace_all(&input, self.delims.field.to_string());

            Cow::Owned(output.into())

            /*
            // TODO: Awesome forwards-only string manip work...   gotta start simple

            let first = first.unwrap().start();
            let mut output: Vec<u8> = Vec::from(input[0..first].as_bytes());
            output.reserve(input.len() - first);

            let rest = input[first+1..].bytes(); // the +1 skips the initial backslash that got us here

            let mut iter = rest.into_iter();

            while let Some(c) = iter.next() {
                match c {
                    b'F' => {
                        output.extend_from_slice(&self.field_bytes);
                        iter.next(); // eat the next slash //TODO: How do we know that was a slash?
                    }
                    _ => output.push(c),
                }
            }

            Cow::Owned(String::from_utf8(output).unwrap())
            */
        } else {
            // no escape char in the string at all, just return what we have
            input
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

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
    fn test_decode_does_nothing_if_backslash_is_not_escape_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"There are no escape sequences here\there."#;
        let output = escaper.decode(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_decode_handles_field_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this \F\ please"#;
        let output = escaper.decode(input);
        assert_eq!(output, "Escape this | please");
    }

    #[test]
    fn ensure_decode_does_not_eat_chars_it_shouldnt() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this \F please"#;
        let output = escaper.decode(input);
        assert_eq!(output, input);
    }

    #[test]
    fn ensure_decode_handles_custom_delims() {
        let delims = Separators::from_str("MSH|!@#$|").unwrap();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this #F# please"#;
        let output = escaper.decode(input);
        assert_eq!(output, "Escape this # please");
    }
}
