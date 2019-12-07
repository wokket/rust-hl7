/// This module parses a whole segment.  It is expected that only a single segment (i.e. one logical line from the HL7) is passed at a time.
pub struct SegmentParser;

use super::field_parser::FieldParser;
use super::*;

impl SegmentParser {
    pub fn parse_segment(input: &str, delims: &Seperators) -> Segment {
        let fields = input
            .trim() //remove leading and trailing berko chars (bigger issue when debugging)
            .split(delims.field) //split by delimiter
            .map(|field_value| FieldParser::parse_field(field_value, delims)) //call the parser for each value
            .collect(); //turn it into a vec

        Segment { fields: fields } //return the new segment
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_basic_field_construction() {
        let input = "Test|Value";
        let expected = Segment {
            fields: vec![
                Field {
                    repeats: vec![Repeat {
                        components: vec!["Test".to_string()],
                    }],
                },
                Field {
                    repeats: vec![Repeat {
                        components: vec!["Value".to_string()],
                    }],
                },
            ],
        };

        let actual = SegmentParser::parse_segment(input, &Seperators::default());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_real_field_construction() {
        let input = "OBR|1|20061019172719||76770^Ultrasound: retroperitoneal^C4|||12349876";
        let expected = Segment {
            fields: vec![
                Field {
                    repeats: vec![Repeat {
                        components: vec!["OBR".to_string()],
                    }],
                },
                Field {
                    repeats: vec![Repeat {
                        components: vec!["1".to_string()],
                    }],
                },
                Field {
                    repeats: vec![Repeat {
                        components: vec!["20061019172719".to_string()],
                    }],
                },
                Field { repeats: vec![] },
                Field {
                    repeats: vec![Repeat {
                        components: vec![
                            "76770".to_string(),
                            "Ultrasound: retroperitoneal".to_string(),
                            "C4".to_string(),
                        ],
                    }],
                },
                Field { repeats: vec![] },
                Field { repeats: vec![] },
                Field {
                    repeats: vec![Repeat {
                        components: vec!["12349876".to_string()],
                    }],
                },
            ],
        };

        let actual = SegmentParser::parse_segment(input, &Seperators::default());
        assert_eq!(expected, actual);
    }
}
