pub mod fields;
pub mod message;
pub mod segments;
pub mod separators;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Hl7ParseError {
    #[fail(display = "Unexpected error: {}", error)]
    Generic { error: String },

    #[fail(
        display = "Failure parsing MSH1/MSH2 while discovering separator chars: {}",
        error
    )]
    Msh1Msh2 { error: String },

    #[fail(display = "Required value missing")]
    MissingRequiredValue //{ /*error: NoneError*/ }, // remmed for #2 until NoneError gets more stable.
}

/*
impl From<NoneError> for Hl7ParseError {
    fn from(error: NoneError) -> Self {
        // this would only be called if we `?` a `None` somewhere.
        Hl7ParseError::MissingRequiredValue { error }
    }
}*/

// /// A repeat of a field is a set of 0 or more sub component values.
// /// Currently all values are stored as their original string representations.  Methods to convert
// /// the values to their HL7-spec types is outside the scope of the parser.
// #[derive(Debug, PartialEq)]
// pub struct Repeat {
//     pub components: Vec<String>,
// }

// /// A Field is a single 'value between the pipes'.
// /// It consists of (0 or more) repeats.
// #[derive(Debug, PartialEq)]
// pub struct Field {
//     pub repeats: Vec<Repeat>,
// }

// /// A single segment, 0x13 delimited line from a source HL7 message consisting of multiple fields.
// #[derive(Debug, PartialEq)]
// pub struct Segment {
//     pub fields: Vec<Field>,
// }

// /// A Message is an entire HL7 message parsed into it's consitituent segments, fields, repeats and subcomponents
// /// It consists of (1 or more) Segments.
// #[derive(Debug, PartialEq)]
// pub struct Message {
//     /// The source string that was parsed to form this message.
//     /// We need our own copy to ensure the &str's are referencing a string that lives long enough in an FFI scenario
//     input: String,
//     pub segments: Vec<Segment>,
// }

// /// A HL7 field can contain multiple 'repeats', eg to support multiple nationalities for a patient.
// impl Repeat {
//     /// Returns all components for this repeat as a single string.  If multiple components are present they are joined
//     /// with the standard HL7 '^' separator.
//     pub fn get_as_string(&self) -> String {
//         let delims = Separators::default();

//         if self.components.len() == 0 {
//             return "".to_string();
//         } else {
//             return self.components.join(delims.component.to_string().as_str()); //TODO: How to convert char to &str in a sane way?
//         }
//     }
// }

//     // pub fn get_segments(&self, segment_type: &str) -> Vec<&Segment> {
//     //     self.segments
//     //         .iter()
//     //         .filter(|segment| {
//     //             let seg_type = segment.fields[0].get_all_as_string();
//     //             //println!("Checking Segment: '{}'", seg_type);
//     //             seg_type == segment_type
//     //         })
//     //         .collect()
//     // }

//     // pub fn get_field(&self, segment_type: &str, field_index: usize) -> String {
//     //     let matching_segments = self.get_segments(segment_type);
//     //     let segment = matching_segments[0];
//     //     let result = segment.fields[field_index].get_all_as_string();
//     //     result
//     // }
// }

// #[cfg(test)]
// mod tests {
//     use super::separators::Separators;
//     use super::*;
//     #[test]
//     fn ensure_separators_load_correctly() {
//         let expected = Separators::default();
//         let actual = Separators::new("MSH|^~\\&|CATH|StJohn|AcmeHIS|StJohn|20061019172719||ACK^O01|MSGID12349876|P|2.3\rMSA|AA|MSGID12349876");

//         assert_eq!(expected.component, actual.component);
//         assert_eq!(expected.escape_char, actual.escape_char);
//         assert_eq!(expected.field, actual.field);
//         assert_eq!(expected.repeat, actual.repeat);
//         assert_eq!(expected.segment, actual.segment);
//         assert_eq!(expected.subcomponent, actual.subcomponent);
//     }

//     #[test]
//     fn repeat_get_all_as_string_single_simple_value() {
//         let r = Repeat {
//             components: vec!["Simple Repeat".to_string()],
//         };

//         let actual = r.get_as_string();
//         assert_eq!(actual, "Simple Repeat");
//     }

//     #[test]
//     fn repeat_get_all_as_string_multi_components() {
//         let r = Repeat {
//             components: vec!["Multiple".to_string(), "Components".to_string()],
//         };

//         let actual = r.get_as_string();
//         assert_eq!(actual, "Multiple^Components");
//     }

//     #[test]
//     fn field_get_all_as_string_single_simple_value() {
//         let f = Field {
//             repeats: vec![Repeat {
//                 components: vec!["Simple Repeat".to_string()],
//             }],
//         };

//         // let actual = f.get_all_as_string();
//         // assert_eq!(actual, "Simple Repeat");
//     }

//     #[test]
//     fn field_get_all_as_string_multiple_repeats() {
//         let f = Field {
//             repeats: vec![
//                 Repeat {
//                     components: vec!["Repeat 1".to_string()],
//                 },
//                 Repeat {
//                     components: vec!["Repeat 2".to_string()],
//                 },
//             ],
//         };

//         // let actual = f.get_all_as_string();
//         // assert_eq!(actual, "Repeat 1~Repeat 2");
//     }

//     #[test]
//     fn test_segment_lookup() {
//         let msg =
//             message_parser::MessageParser::parse_message("MSH|fields\rOBR|segment\r".to_string()); //note the trailing \r
//                                                                                                    /*let expected = Message {
//                                                                                                        segments: vec![
//                                                                                                            Segment {
//                                                                                                                fields: vec![
//                                                                                                                    Field {
//                                                                                                                        repeats: vec![Repeat {
//                                                                                                                            sub_components: vec!["test"],
//                                                                                                                        }],
//                                                                                                                    },
//                                                                                                                    Field {
//                                                                                                                        repeats: vec![Repeat {
//                                                                                                                            sub_components: vec!["fields"],
//                                                                                                                        }],
//                                                                                                                    },
//                                                                                                                ],
//                                                                                                            },
//                                                                                                            Segment {
//                                                                                                                fields: vec![
//                                                                                                                    Field {
//                                                                                                                        repeats: vec![Repeat {
//                                                                                                                            sub_components: vec!["another"],
//                                                                                                                        }],
//                                                                                                                    },
//                                                                                                                    Field {
//                                                                                                                        repeats: vec![Repeat {
//                                                                                                                            sub_components: vec!["segment"],
//                                                                                                                        }],
//                                                                                                                    },
//                                                                                                                ],
//                                                                                                            },
//                                                                                                        ],
//                                                                                                    };*/
//         let expected = Segment {
//             fields: vec![
//                 Field {
//                     repeats: vec![Repeat {
//                         components: vec!["OBR".to_string()],
//                     }],
//                 },
//                 Field {
//                     repeats: vec![Repeat {
//                         components: vec!["segment".to_string()],
//                     }],
//                 },
//             ],
//         };

//         // let result = msg.get_segments("OBR");
//         // assert!(result.len() == 1);
//         // assert_eq!(expected, *result[0]);
//     }
// }
