/// This module parses a whole message.  It is expected that only a single message is passed at a time.
/// Note that this parses to constituent values, but makes no effort to intepret those values (ie no strong-typing of segments etc)
/// or to interpret the values (coercian to numeric values etc).
pub struct MessageParser;

use super::segment_parser::SegmentParser;
use super::*;

impl MessageParser {
    /// Parses an entire HL7 message into it's component values
    pub fn parse_message<'a>(input: &'a str) -> Message<'a> {
        let mut segments = Vec::new();

        for segment_value in input.split('\r') {
            if segment_value.len() == 0 {
                //we've hit the end-of-message blank line delimnter, proceed no further
                break;
            }

            let segment = SegmentParser::parse_segment(segment_value);
            segments.push(segment);
        }

        Message { segments: segments }
    }

    pub fn parse_message_alt<'a>(input: &'a str) -> Message<'a> {
        let mut segments = Vec::with_capacity(5);

        for segment_value in input.split('\r') {
            if segment_value.len() == 0 {
                //we've hit the end-of-message blank line delimnter, proceed no further
                break;
            }

            let segment = SegmentParser::parse_segment_alt(segment_value);
            segments.push(segment);
        }

        Message { segments: segments }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_message() {
        let result = MessageParser::parse_message("test|fields\ranother|segment");
        let expected = Message {
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
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_message_with_final_delimiter() {
        let result = MessageParser::parse_message("test|fields\ranother|segment\r"); //note the trailing \r
        let expected = Message {
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
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_message_with_message_delimiter_included() {
        let result = MessageParser::parse_message("test|fields\ranother|segment\r\r"); //note the trailing \r\r
        let expected = Message {
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

    #[bench]
    fn bench_full_message_alternate(b: &mut test::Bencher) {
        //comparitor for a/b testing
        b.iter(
            || MessageParser::parse_message_alt(get_sample_message()), //note the trailing \r\r
        );
    }

    fn get_sample_message() -> &'static str {
        "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F"
    }

}
