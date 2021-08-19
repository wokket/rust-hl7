use super::{fields::Field, separators::Separators, Hl7ParseError};
use std::fmt::Display;
use std::ops::Index;

/// All defined segment types
pub enum Segments<'a> {
    Generic(Segment<'a>),
    MSH(MshSegment<'a>),
}

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq, Clone)]
pub struct Segment<'a> {
    pub source: &'a str,
    pub delim: char,
    pub fields: Vec<Field<'a>>,
}

impl<'a> Segment<'a> {
    /// Convert the given line of text into a Segment.
    pub fn parse<S: Into<&'a str>>(
        input: S,
        delims: &Separators,
    ) -> Result<Segment<'a>, Hl7ParseError> {
        let input = input.into();

        let fields: Result<Vec<Field<'a>>, Hl7ParseError> = input
            .split(delims.field)
            .map(|line| Field::parse(line, delims))
            .collect();

        let fields = fields?;
        let seg = Segment {
            source: input,
            delim: delims.segment,
            fields,
        };
        Ok(seg)
    }

    /// Export source to str
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.source
    }

    /// Access Field as string reference
    pub fn query<'b, S>(&self, fidx: S) -> &'a str
    where
        S: Into<&'b str>,
    {
        let fidx = fidx.into();
        let sections = fidx.split('.').collect::<Vec<&str>>();

        match sections.len() {
            1 => {
                let stringnum = sections[0]
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                self[idx]
            }
            _ => {
                let stringnum = sections[0]
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                let field = &self.fields[idx];
                let query = sections[1..].join(".");

                field.query(&*query)
            }
        }
    }
}

impl<'a> Display for Segment<'a> {
    /// Required for to_string() and other formatter consumers
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl<'a> Index<usize> for Segment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    fn index(&self, fidx: usize) -> &Self::Output {
        if fidx > self.fields.len() - 1 {
            return &"";
        };
        &self.fields[fidx].source
    }
}

impl<'a> Index<(usize, usize)> for Segment<'a> {
    type Output = &'a str;
    /// Access Field component as string reference
    fn index(&self, fidx: (usize, usize)) -> &Self::Output {
        if fidx.0 > self.fields.len() - 1 || fidx.1 > self.fields[fidx.0].components.len() - 1 {
            return &"";
        }
        &self.fields[fidx.0][fidx.1]
    }
}

impl<'a> Index<(usize, usize, usize)> for Segment<'a> {
    type Output = &'a str;
    /// Access Field subcomponent as string reference
    fn index(&self, fidx: (usize, usize, usize)) -> &Self::Output {
        if fidx.0 > self.fields.len() - 1
            || fidx.1 > self.fields[fidx.0].components.len() - 1
            || fidx.2 > self.fields[fidx.0].subcomponents[fidx.1].len() - 1
        {
            return &"";
        }
        &self.fields[fidx.0][(fidx.1, fidx.2)]
    }
}

#[cfg(feature = "string_index")]
impl<'a> Index<String> for Segment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    #[cfg(feature = "string_index")]
    fn index(&self, fidx: String) -> &Self::Output {
        let sections = fidx.split('.').collect::<Vec<&str>>();
        let stringnum = sections[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>();
        let mut idx: usize = stringnum.parse().unwrap();
        // MSH segment has an off-by-one problem in that the first
        // field separator is considered to be a field in the spec
        // https://hl7-definition.caristix.com/v2/HL7v2.8/Segments/MSH
        if self.fields[0].source == "MSH" {
            if idx == 1 {
                return &"|" //&&self.source[3..3] //TODO figure out how to return a string ref safely
            } else {
                idx = idx - 1
            }
        }
        match sections.len() {
            1 => {
                &self[idx]
            }
            _ => {
                &self.fields[idx][sections[1..].join(".")]
            }
        }
    }
}

#[cfg(feature = "string_index")]
impl<'a> Index<&str> for Segment<'a> {
    type Output = &'a str;

    /// Access Segment, Field, or sub-field string references by string index
    #[cfg(feature = "string_index")]
    fn index(&self, idx: &str) -> &Self::Output {
        &self[String::from(idx)]
    }
}

/// The most important Segment, almost all HL7 messages have an MSH (MLLP simple ack I'm looking at you).
/// Given the importance of this segment for driving application behaviour, it gets the special treatment
/// of a fully typed segment, not just a bag of fields....
#[derive(Debug, PartialEq)]
pub struct MshSegment<'a> {
    pub source: &'a str,
    //this initial layout largely stolen from the _other_ hl7 crate: https://github.com/njaremko/hl7
    pub msh_1_field_separator: char,
    pub msh_2_encoding_characters: Separators,
    pub msh_3_sending_application: Option<Field<'a>>,
    pub msh_4_sending_facility: Option<Field<'a>>,
    pub msh_5_receiving_application: Option<Field<'a>>,
    pub msh_6_receiving_facility: Option<Field<'a>>,
    pub msh_7_date_time_of_message: Field<'a>,
    pub msh_8_security: Option<Field<'a>>,
    pub msh_9_message_type: Field<'a>,
    pub msh_10_message_control_id: Field<'a>,
    pub msh_11_processing_id: Field<'a>,
    pub msh_12_version_id: Field<'a>,
    pub msh_13_sequence_number: Option<Field<'a>>,
    pub msh_14_continuation_pointer: Option<Field<'a>>,
    pub msh_15_accept_acknowledgment_type: Option<Field<'a>>,
    pub msh_16_application_acknowledgment_type: Option<Field<'a>>,
    pub msh_17_country_code: Option<Field<'a>>,
    pub msh_18_character_set: Option<Field<'a>>, //TODO: repeating field
    pub msh_19_principal_language_of_message: Option<Field<'a>>,
    // pub msh_20_alternate_character_set_handling_scheme: Option<Field<'a>>,
    // pub msh_21_message_profile_identifier: Option<Vec<Field<'a>>>,
    // pub msh_22_sending_responsible_organization: Option<Field<'a>>,
    // pub msh_23_receiving_responsible_organization: Option<Field<'a>>,
    // pub msh_24_sending_network_address: Option<Field<'a>>,
    // pub msh_25_receiving_network_address: Option<Field<'a>>,
}

impl<'a> MshSegment<'a> {
    pub fn parse<S: Into<&'a str>>(
        input: S,
        delims: &Separators,
    ) -> Result<MshSegment<'a>, Hl7ParseError> {
        let input = input.into();

        let mut fields = input.split(delims.field);

        assert!(fields.next().unwrap() == "MSH");

        let _ = fields.next(); //consume the delimiter chars

        let msh = MshSegment {
            source: input,
            msh_1_field_separator: delims.field,
            msh_2_encoding_characters: delims.to_owned(),
            msh_3_sending_application: Field::parse_optional(fields.next(), delims)?,
            msh_4_sending_facility: Field::parse_optional(fields.next(), delims)?,
            msh_5_receiving_application: Field::parse_optional(fields.next(), delims)?,
            msh_6_receiving_facility: Field::parse_optional(fields.next(), delims)?,
            msh_7_date_time_of_message: Field::parse_mandatory(fields.next(), delims)?,
            msh_8_security: Field::parse_optional(fields.next(), delims)?,
            msh_9_message_type: Field::parse_mandatory(fields.next(), delims)?,
            msh_10_message_control_id: Field::parse_mandatory(fields.next(), delims)?,
            msh_11_processing_id: Field::parse_mandatory(fields.next(), delims)?,
            msh_12_version_id: Field::parse_mandatory(fields.next(), delims)?,
            msh_13_sequence_number: Field::parse_optional(fields.next(), delims)?,
            msh_14_continuation_pointer: Field::parse_optional(fields.next(), delims)?,
            msh_15_accept_acknowledgment_type: Field::parse_optional(fields.next(), delims)?,
            msh_16_application_acknowledgment_type: Field::parse_optional(fields.next(), delims)?,
            msh_17_country_code: Field::parse_optional(fields.next(), delims)?,
            msh_18_character_set: Field::parse_optional(fields.next(), delims)?,
            msh_19_principal_language_of_message: Field::parse_optional(fields.next(), delims)?,
        };

        Ok(msh)
    }

    /// Export source to str
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.source
    }
}

impl<'a> Display for MshSegment<'a> {
    /// Required for to_string() and other formatter consumers
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl<'a> Clone for MshSegment<'a> {
    /// Creates a new Message object using _the same source_ slice as the original.
    fn clone(&self) -> Self {
        let delims = self.msh_2_encoding_characters;
        MshSegment::parse(self.source, &delims).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{message::Message, segments::*, separators::Separators, Hl7ParseError};
    use std::convert::TryFrom;

    #[test]
    fn ensure_numeric_index() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7).unwrap();
        let x = &msg.segments[1];
        let (f, c, s) = (x[1], x[(1, 1)], x[(1, 1, 0)]);
        assert_eq!(f, "segment^sub&segment");
        assert_eq!(c, "sub&segment");
        assert_eq!(s, "sub");
    }
    #[cfg(feature = "string_index")]
    mod string_index_tests {
        use super::*;
        #[test]
        fn ensure_string_index() {
            let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
            let msg = Message::try_from(hl7).unwrap();
            let x = &msg.segments[1];
            let (f, c, s, oob) = (
                x.query("F1"),                       //&str
                x.query("F1.R2"),                    // &str
                x.query(&*String::from("F1.R2.C1")), //String
                String::from(x.query("F10")) + x.query("F1.R10") + x.query("F1.R2.C10"),
            );
            assert_eq!(f, "segment^sub&segment");
            assert_eq!(c, "sub&segment");
            assert_eq!(s, "sub");
            assert_eq!(oob, "");
        }
    }

    #[test]
    fn ensure_msh_fields_are_populated() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4";
        let delims = Separators::default();

        let msh = MshSegment::parse(hl7, &delims)?;

        assert_eq!(msh.msh_1_field_separator, '|');

        let msh3 = msh
            .msh_3_sending_application
            .ok_or(Hl7ParseError::Generic("Parse Error".to_string()))?;
        assert_eq!(msh3.value(), "GHH LAB");

        let msh4 = msh
            .msh_4_sending_facility
            .ok_or(Hl7ParseError::Generic("Parse Error".to_string()))?;
        assert_eq!(msh4.value(), "ELAB-3");

        let msh5 = msh
            .msh_5_receiving_application
            .ok_or(Hl7ParseError::Generic("Parse Error".to_string()))?;
        assert_eq!(msh5.value(), "GHH OE");

        let msh6 = msh
            .msh_6_receiving_facility
            .ok_or(Hl7ParseError::Generic("Parse Error".to_string()))?;
        assert_eq!(msh6.value(), "BLDG4");

        assert_eq!(msh.msh_8_security, None); //blank field check
        assert_eq!(msh.msh_12_version_id.value(), "2.4"); //we got to the end ok
        Ok(())
    }

    #[test]
    fn ensure_msh_clones_correctly() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4";
        let delims = Separators::default();

        let msh = MshSegment::parse(hl7, &delims)?;
        let dolly = msh.clone();
        assert_eq!(msh, dolly);
        Ok(())
    }
}
