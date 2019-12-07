//! This module parses a whole pipe-delimited style HL7 V2 message.  It is expected that only a single message is passed at a time.
//! Note that this parses to constituent values, but makes no effort to interpret those values (ie no strong-typing of segments etc)
//! or to interpret the values (coercion to numeric values etc).  Utility API's [are being added](Field::get_as_string) to better handle these fields
pub struct MessageParser;

use super::*;

impl MessageParser {
    /// Parses an entire HL7 message into it's component values
    pub fn parse_message(input: String) -> Message {
        let mut result = Message::new(input);

        result.build_from_input();

        result
    }
}

#[cfg(test)]

mod tests {
    //use super::segment_parser::SegmentParser;
    use super::*;

    #[test]
    fn test_basic_message() {
        let result = MessageParser::parse_message("MSH|^~\\&|fields\ranother|segment".to_string());
        let expected = Message {
            input: "MSH|^~\\&|fields\ranother|segment".to_string(),
            segments: vec![
                Segment {
                    fields: vec![
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["MSH".to_string()],
                            }],
                        },
                        Field {
                            repeats: vec![
                                Repeat {
                                    components: vec!["".to_string(), "".to_string()],
                                },
                                Repeat {
                                    components: vec!["\\&".to_string()],
                                },
                            ],
                        },
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["fields".to_string()],
                            }],
                        },
                    ],
                },
                Segment {
                    fields: vec![
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["another".to_string()],
                            }],
                        },
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["segment".to_string()],
                            }],
                        },
                    ],
                },
            ],
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_message_with_final_delimiter() {
        let result =
            MessageParser::parse_message("MSH|^~\\&|fields\ranother|segment\r".to_string()); //note the trailing \r
        let expected = Message {
            input: "MSH|^~\\&|fields\ranother|segment\r".to_string(),
            segments: vec![
                Segment {
                    fields: vec![
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["MSH".to_string()],
                            }],
                        },
                        Field {
                            repeats: vec![
                                Repeat {
                                    components: vec!["".to_string(), "".to_string()],
                                },
                                Repeat {
                                    components: vec!["\\&".to_string()],
                                },
                            ],
                        },
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["fields".to_string()],
                            }],
                        },
                    ],
                },
                Segment {
                    fields: vec![
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["another".to_string()],
                            }],
                        },
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["segment".to_string()],
                            }],
                        },
                    ],
                },
            ],
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_message_with_message_delimiter_included() {
        let result =
            MessageParser::parse_message("MSH|^~\\&|fields\ranother|segment\r\r".to_string()); //note the trailing \r\r
        let expected = Message {
            input: "MSH|^~\\&|fields\ranother|segment\r\r".to_string(),
            segments: vec![
                Segment {
                    fields: vec![
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["MSH".to_string()],
                            }],
                        },
                        Field {
                            repeats: vec![
                                Repeat {
                                    components: vec!["".to_string(), "".to_string()],
                                },
                                Repeat {
                                    components: vec!["\\&".to_string()],
                                },
                            ],
                        },
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["fields".to_string()],
                            }],
                        },
                    ],
                },
                Segment {
                    fields: vec![
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["another".to_string()],
                            }],
                        },
                        Field {
                            repeats: vec![Repeat {
                                components: vec!["segment".to_string()],
                            }],
                        },
                    ],
                },
            ],
        };

        assert_eq!(expected, result);
    }

    


}
