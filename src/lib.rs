#![feature(test)]

extern crate itertools;
extern crate libc;
extern crate test;

mod field_parser;
pub mod message_parser;
pub mod native;
mod segment_parser;

use itertools::Itertools;

/// A repeat of a field is a set of 0 or more sub component values.
/// Currently all values are stored as their original string representations.  Methods to convert
/// the values to their HL7-spec types is outside the scope of the parser.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Repeat {
    pub sub_components: Vec<String>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Field {
    pub repeats: Vec<Repeat>,
}

/// A single segment, 0x13 delimited line from a source HL7 message consisting of multiple fields.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Segment {
    pub fields: Vec<Field>,
}

/// A Message is an entire HL7 message parsed into it's consitituent segments, fields, repeats and subcomponents
/// It consists of (1 or more) Segments.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Message {
    /// The source string that was parsed to form this message.
    /// We need our own copy to ensure the &str's are referencing a string that lives long enough in an FFI scenario
    input: String,
    pub segments: Vec<Segment>,
}

/// A HL7 field can contain multiple 'repeats', eg to support multiple nationalities for a patient.
impl Repeat {
    /// Returns all subcomponents for this repeat as a single string.  If multiple subcomponents are present they are joined
    /// with the standard HL7 '^' separator.
    pub fn get_as_string(&self) -> String {
        if self.sub_components.len() == 0 {
            return "".to_string();
        } else {
            return self.sub_components.join("^");
        }
    }
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
impl Field {
    /// Returns a single String built from all the repeats.segment_parser
    /// This value includes the standard '~' HL7 delimiter between repeats.
    /// A copy  of the original data is made here (rather than returning &str) as the value is expected to be returned to external callers who
    /// shouldn't have to keep the entire source message alive
    pub fn get_all_as_string(&self) -> String {
        if self.repeats.len() == 0 {
            return "".to_string();
        }

        self.repeats.iter().map(|r| r.get_as_string()).join("~")
    }
}

impl Message {
    pub fn new(input: String) -> Message {
        Message {
            input: input,
            segments: Vec::new(),
        }
    }

    pub fn build_from_input(&mut self) {
        let iter = self.input.split('\r');

        for segment_value in iter {
            if segment_value.len() == 0 {
                //we've hit the end-of-message blank line delimnter, proceed no further
                break;
            }

            let segment = segment_parser::SegmentParser::parse_segment(segment_value);
            self.segments.push(segment);
        }
    }

    pub fn get_segments(&self, segment_type: &str) -> Vec<&Segment> {
        self.segments
            .iter()
            .filter(|segment| {
                let seg_type = segment.fields[0].get_all_as_string();
                //println!("Checking Segment: '{}'", seg_type);
                seg_type == segment_type
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_get_all_as_string_single_simple_value() {
        let r = Repeat {
            sub_components: vec!["Simple Repeat".to_string()],
        };

        let actual = r.get_as_string();
        assert_eq!(actual, "Simple Repeat");
    }

    #[test]
    fn repeat_get_all_as_string_multi_components() {
        let r = Repeat {
            sub_components: vec!["Multiple".to_string(), "Components".to_string()],
        };

        let actual = r.get_as_string();
        assert_eq!(actual, "Multiple^Components");
    }

    #[test]
    fn field_get_all_as_string_single_simple_value() {
        let f = Field {
            repeats: vec![Repeat {
                sub_components: vec!["Simple Repeat".to_string()],
            }],
        };

        let actual = f.get_all_as_string();
        assert_eq!(actual, "Simple Repeat");
    }

    #[test]
    fn field_get_all_as_string_multiple_repeats() {
        let f = Field {
            repeats: vec![
                Repeat {
                    sub_components: vec!["Repeat 1".to_string()],
                },
                Repeat {
                    sub_components: vec!["Repeat 2".to_string()],
                },
            ],
        };

        let actual = f.get_all_as_string();
        assert_eq!(actual, "Repeat 1~Repeat 2");
    }

    #[test]
    fn test_segment_lookup() {
        let msg =
            message_parser::MessageParser::parse_message("MSH|fields\rOBR|segment\r".to_string()); //note the trailing \r
                                                                                                   /*let expected = Message {
                                                                                                       segments: vec![
                                                                                                           Segment {
                                                                                                               fields: vec![
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["test"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["fields"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                               ],
                                                                                                           },
                                                                                                           Segment {
                                                                                                               fields: vec![
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["another"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                                   Field {
                                                                                                                       repeats: vec![Repeat {
                                                                                                                           sub_components: vec!["segment"],
                                                                                                                       }],
                                                                                                                   },
                                                                                                               ],
                                                                                                           },
                                                                                                       ],
                                                                                                   };*/

        let expected = Segment {
            fields: vec![
                Field {
                    repeats: vec![Repeat {
                        sub_components: vec!["OBR".to_string()],
                    }],
                },
                Field {
                    repeats: vec![Repeat {
                        sub_components: vec!["segment".to_string()],
                    }],
                },
            ],
        };

        let result = msg.get_segments("OBR");
        assert!(result.len() == 1);
        assert_eq!(expected, *result[0]);
    }
}
