extern crate itertools;
extern crate test;

mod field_parser;
pub mod message_parser;
mod segment_parser;

use itertools::Itertools;

/// A repeat of a field is a set of 0 or more sub component values.
/// Currently all values are stored as their original string representations.  Methods to convert
/// the values to their HL7-spec types is outside the scope of the parser.
#[derive(Debug, PartialEq)]
pub struct Repeat<'a> {
    pub sub_components: Vec<&'a str>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub repeats: Vec<Repeat<'a>>,
}

/// A Field is a single 'value between the pipes'.
/// It consists of (0 or more) repeats.
#[derive(Debug, PartialEq)]
pub struct Segment<'a> {
    pub fields: Vec<Field<'a>>,
}

/// A Message is an entire HL7 message parsed into it's consitituent segments, fields, repeats and subcomponents
/// It consists of (1 or more) Segments.
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub segments: Vec<Segment<'a>>,
}

impl<'a> Repeat<'a> {
    pub fn get_as_string(&self) -> String {
        if self.sub_components.len() == 0 {
            return "".to_string();
        } else {
            return self.sub_components.join("^");
        }
    }
}

impl<'a> Field<'a> {
    /// Returns a single String built from all the repeats.segment_parser
    /// This value includes HL7 delimiter values between repeats, components and sub components.segment_parser
    /// A copy  of the oringla data is made here (rather than a &str) as the value is expected to be returned to external callers who
    /// shouldn't have to keep the entire source message alive
    pub fn get_all_as_string(&self) -> String {
        if self.repeats.len() == 0 {
            return "".to_string();
        }

        self.repeats.iter().map(|r| r.get_as_string()).join("~")
    }
}

impl<'a> Message<'a> {
    pub fn get_segments(&self, segment_type: &str) -> Vec<&Segment<'a>> {
        self.segments
            .iter()
            .filter(|segment| segment.fields[0].get_all_as_string() == segment_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_get_all_as_string_single_simple_value() {
        let r = Repeat {
            sub_components: vec!["Simple Repeat"],
        };

        let actual = r.get_as_string();
        assert_eq!(actual, "Simple Repeat");
    }

    #[test]
    fn repeat_get_all_as_string_multi_components() {
        let r = Repeat {
            sub_components: vec!["Multiple", "Components"],
        };

        let actual = r.get_as_string();
        assert_eq!(actual, "Multiple^Components");
    }

    #[test]
    fn field_get_all_as_string_single_simple_value() {
        let f = Field {
            repeats: vec![Repeat {
                sub_components: vec!["Simple Repeat"],
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
                    sub_components: vec!["Repeat 1"],
                },
                Repeat {
                    sub_components: vec!["Repeat 2"],
                },
            ],
        };

        let actual = f.get_all_as_string();
        assert_eq!(actual, "Repeat 1~Repeat 2");
    }

    #[test]
    fn test_segment_lookup() {
        let msg = message_parser::MessageParser::parse_message("MSH|fields\rOBR|segment\r"); //note the trailing \r
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
                        sub_components: vec!["OBR"],
                    }],
                },
                Field {
                    repeats: vec![Repeat {
                        sub_components: vec!["segment"],
                    }],
                },
            ],
        };

        let result = msg.get_segments("OBR");
        assert!(result.len() == 1);
        assert_eq!(expected, *result[0]);
    }
}
