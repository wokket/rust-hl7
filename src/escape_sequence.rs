use log::{debug, trace};
use regex::Regex;

use crate::separators::Separators;
use std::borrow::Cow;

/// This struct provides the decoding functionality to parse escape sequences from the source string back to their original chars.
///
/// For more info see [here](https://www.lyniate.com/knowledge-hub/hl7-escape-sequences/) or [here](https://confluence.hl7australia.com/display/OOADRM20181/Appendix+1+Parsing+HL7v2#Appendix1ParsingHL7v2-Dealingwithreservedcharactersanddelimiters)
///
/// ## Example:
/// ```
/// # use rusthl7::escape_sequence::EscapeSequence;
/// # use rusthl7::separators::Separators;
/// let delims = Separators::default();
/// let decoder = EscapeSequence::new(delims);
/// let hl7_field_value = r#"Obstetrician \T\ Gynaecologist"#;
/// let decoded = decoder.decode(hl7_field_value);
/// assert_eq!(decoded, r#"Obstetrician & Gynaecologist"#);
/// ```
///
/// ## Details
///
/// This decoder will replace some, **but not all** of the standard HL7 escape sequences. `\E\`,`\F\`, '\R\`, `\S\`, `\T\` are all handled, and replaced with the Escape, Field, Repeat, Component and Sub-Component separator chars respectively
/// 
/// The following sequences are **NOT** replaced by design and will be left in the string:
/// - `\H\` Indicates the start of highlighted text, this is a consuming application problem and will not be replaced.
/// - `\N\` Indicates the end of highlighted text and resumption of normal text.  This is a consuming application problem and will not be replaced.
/// - `\Z...\` Custom application escape sequences, these are custom (as are most `Z` items in HL7) and will not be replaced.
///
/// Also, not all of the sequences that _should_ be replaced are currently being handled, specifically:
/// /// - `\Cxxyy\`, '\Mxxyyzz\, '\Xdd..\` _should_ be handled, but aren't currently.  There's [some suggestion](https://confluence.hl7australia.com/display/OOADRM20181/Appendix+1+Parsing+HL7v2#Appendix1ParsingHL7v2-Unicodecharacters) that these are discouraged in lieu of html-escaped values
///
/// If there's _no possibility_ of escape sequences (because there's no escape characters, typically backslashes) in the value, this function short circuits as early as possible and returns the original string slice for optimum performance.
pub struct EscapeSequence {
    escape_buf: [u8; 1],
    field_buf: [u8; 1],
    repeat_buf: [u8; 1],
    component_buf: [u8; 1],
    subcomponent_buf: [u8; 1],
    escape_regex: Regex,
}

impl<'a> EscapeSequence {
    /// Create a new struct ready for processing of escape sequences.
    /// Escape sequences in HL7 are dependent on the actual delimiters used _for that message_, and so we need a [Separators] instance to know what chars we're working with.
    ///
    /// Creating a new [EscapeSequence] does involve some non-trivial work in order to improve the performance of the `decode()` operations.  It's expected that instances of this struct will be cached
    /// per message, or per sending application if it will always use the same separators, or for the lifetime of the process if you're only dealing with known (often default) separators.
    pub fn new(delims: Separators) -> EscapeSequence {
        let regex = if delims.escape_char == '\\' {
            Regex::new(r#"\\"#) // needs special handling because backslashes have meaning in regexes, and need to be escaped
        } else {
            Regex::new(String::from(delims.escape_char).as_str()) //everything else just works (I hope!)
        }
        .unwrap();

        let mut return_val = EscapeSequence {
            escape_buf: [0; 1], // The spec specifically requires single byte (actually 7-bit ASCII) delim chars
            field_buf: [0; 1],
            repeat_buf: [0; 1],
            component_buf: [0; 1],
            subcomponent_buf: [0; 1],
            escape_regex: regex,
        };

        // We need &str to inject into the output buffer, convert the `Char` here
        let _bytes = delims.escape_char.encode_utf8(&mut return_val.escape_buf);
        let _bytes = delims.field.encode_utf8(&mut return_val.field_buf);
        let _bytes = delims.repeat.encode_utf8(&mut return_val.repeat_buf);
        let _bytes = delims.component.encode_utf8(&mut return_val.component_buf);
        let _bytes = delims
            .subcomponent
            .encode_utf8(&mut return_val.subcomponent_buf);

        return_val
    }

    /// This is where the magic happens.  Call this to update any escape sequences in the given &str.
    pub fn decode<S>(&self, input: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        // The comments below will almost certainly reference backslashes as that is by far the most common escape character
        // the reality is any reference to "backslash" is actually referencing the escape char in the MSH segemnt, and stored in `self.delims.escape_char`

        let input = input.into();
        let first = self.escape_regex.find(&input); //using `regex.find` here is about twice as fast for the 'no sequences' benchmark as using &str.find()...

        match first {
            Some(first) => {
                let first = first.start();

                // We know there's a backslash, so we need to process stuff

                // we're going to be replacing (mainly) 3 char escape sequences (eg `\F\`) with a single char sequence (eg `|`) so the initial length of the input should be sufficient
                let mut output: Vec<u8> = Vec::with_capacity(input.len());
                output.extend_from_slice(input[0..first].as_bytes()); // this doesn't include the escape char we found

                // index in input that we're up to
                let mut i = first;

                debug!("Found first escape char at {}", first);

                while i < input.len() {
                    let start_of_sequence = self.escape_regex.find(&input[i..]);
                    if start_of_sequence.is_none() {
                        // there's nothing left to process, no more backslashes in the rest of the buffer

                        trace!("No more sequence starts in input, completing...");
                        output.extend_from_slice(input[i..].as_bytes()); // add the rest of the input
                        break; // break out of while loop
                    }

                    let start_index = start_of_sequence.unwrap().start() + i; // index is offset into input by i chars as that's what's we subsliced above
                    trace!("Found the next escape char at {}", start_index);

                    let end_of_sequence = self.escape_regex.find(&input[start_index + 1..]);

                    if end_of_sequence.is_none() {
                        // there's nothing left to process, the backslash we are curently looking at is NOT an escape sequence
                        trace!("No more sequence ends in input, completing...");
                        output.extend_from_slice(input[start_index..].as_bytes()); // add the rest of the input (including the escape char that brought us here) in one go
                        break; // break out of while loop
                    }

                    // else we have found another escape char, get the slice in between
                    let end_index = end_of_sequence.unwrap().start() + start_index + 1; // the end is the number of chars after the start_index, not from the start of input
                    trace!("Found end of sequence at {}", end_index);

                    let sequence = &input[start_index + 1..end_index];
                    trace!("Found escape sequence: '{}'", sequence);

                    // we have a possible window of data between i and start_index that we've just read through as text, but isn't yet in output... append it now
                    output.extend_from_slice(input[i..start_index].as_bytes());

                    match sequence {
                        "E" => output.extend_from_slice(&self.escape_buf),
                        "F" => output.extend_from_slice(&self.field_buf),
                        "R" => output.extend_from_slice(&self.repeat_buf),
                        "S" => output.extend_from_slice(&self.component_buf),
                        "T" => output.extend_from_slice(&self.subcomponent_buf),

                        // Highlighted/Normal text sequences need to remain for consuming libraries to act on as they see fit
                        "H" | "N" => {
                            output.extend_from_slice(&self.escape_buf);
                            output.extend_from_slice(sequence.as_bytes());
                            output.extend_from_slice(&self.escape_buf);
                        }

                        _ => {
                            if sequence.starts_with('Z') {
                                println!("Into custom escape sequence, ignoring...");
                                output.extend_from_slice(&self.escape_buf);
                                output.extend_from_slice(sequence.as_bytes());
                                output.extend_from_slice(&self.escape_buf);

                            // TODO: Add more sequences
                            } else {
                                // not a known sequence, must just be two backslashes randomly in a string
                                trace!("Unknown sequence, extending output...");
                                output.extend_from_slice(
                                    input[start_index - 1..end_index].as_bytes(),
                                );
                                // include both the initial escape char, and also the final one.
                            }
                        }
                    }

                    i = end_index + 1; // move through buffer, we we've covered everything up to this point now
                } // while more chars in input to loop through

                Cow::Owned(String::from_utf8(output).unwrap())
            }
            None => {
                // no escape char in the string at all, just return what we have
                input
            }
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

        let input = "There are no escape sequences here/there/.";
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
        let delims = Separators::from_str("MSH^!@#$").unwrap();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this #F# please"#;
        let output = escaper.decode(input);
        assert_eq!(output, "Escape this ^ please");
    }

    #[test]
    fn ensure_decode_handles_eescape_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this \E\ please"#; // convert the escape sequence
        let output = escaper.decode(input);
        assert_eq!(output, r#"Escape this \ please"#); // into a single escape char

        // ensure it moves on past the char it just added
        let input = r#"Escape this \E\ pretty \F\ please"#; // convert the escape sequence
        let output = escaper.decode(input);
        assert_eq!(output, r#"Escape this \ pretty | please"#); // into a single escape char and still handle future sequences ok
    }

    #[test]
    fn test_decode_handles_repeat_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this \R\ please"#;
        let output = escaper.decode(input);
        assert_eq!(output, "Escape this ~ please");
    }

    #[test]
    fn test_decode_handles_component_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Escape this \S\ please"#;
        let output = escaper.decode(input);
        assert_eq!(output, "Escape this ^ please");
    }

    #[test]
    fn test_decode_handles_subcomponent_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Obstetrician \T\ Gynaecologist"#;
        let output = escaper.decode(input);
        assert_eq!(output, "Obstetrician & Gynaecologist");
    }

    #[test]
    fn ensure_decode_ignores_highlighting_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Don't escape this \H\highlighted text\N\ please"#;
        let output = escaper.decode(input);
        assert_eq!(output, input);
    }

    #[test]
    fn ensure_decode_ignores_custom_sequence() {
        let delims = Separators::default();
        let escaper = EscapeSequence::new(delims);

        let input = r#"Don't escape this custom sequence \Z1234\ please"#;
        let output = escaper.decode(input);
        assert_eq!(output, input);
    }
}
