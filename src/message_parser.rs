//! This module parses a whole pipe-delimited style HL7 V2 message.  It is expected that only a single message is passed at a time.
//! Note that this parses to constituent values, but makes no effort to intepret those values (ie no strong-typing of segments etc)
//! or to interpret the values (coercian to numeric values etc).  Utility API's [are being added](Field::get_as_string) to better handle these fields
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

    #[bench]
    fn bench_full_message(b: &mut test::Bencher) {
        b.iter(
            || MessageParser::parse_message(get_sample_message()), //note the trailing \r\r
        );
    }

    #[bench]
    fn message_parse_and_retrieve_field(b: &mut test::Bencher) {
        b.iter(|| {
            let msg = MessageParser::parse_message(get_sample_message());
            let _ = msg.get_segments("OBR")[0].fields[16].get_all_as_string();
        });
    }

    fn get_sample_message() -> String {
        "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F".to_string()
    }

}
