use super::fields::Field;
use super::separators::Separators;
use super::*;

/// The most important Segment, almost all HL7 messages have an MSH (MLLP simple ack I'm looking at you).
/// Given the importance of this segment for driving application behaviour, it gets the special treatment
/// of a fully typed segment, not just a bag of fields....
#[derive(Debug, PartialEq, Clone)]
pub struct MshSegment<'a> {
    pub fields: Vec<Field<'a>>,
    // //this initial layout largely stolen from the _other_ hl7 crate: https://github.com/njaremko/hl7
    msh_1_field_separator: char,
    msh_2_encoding_characters: Separators,
    msh_3_sending_application: Option<Field<'a>>,
    msh_4_sending_facility: Option<Field<'a>>,
    msh_5_receiving_application: Option<Field<'a>>,
    msh_6_receiving_facility: Option<Field<'a>>,
    msh_7_date_time_of_message: Field<'a>,
    // msh_8_security: Option<Field<'a>>,
    // msh_9_message_type: Field<'a>,
    // msh_10_message_control_id: Field<'a>,
    // msh_11_processing_id: Field<'a>,
    // msh_12_version_id: Field<'a>,
    // msh_13_sequence_number: Option<Field<'a>>,
    // msh_14_continuation_pointer: Option<Field<'a>>,
    // msh_15_accept_acknowledgment_type: Option<Field<'a>>,
    // msh_16_application_acknowledgment_type: Option<Field<'a>>,
    // msh_17_country_code: Option<Field<'a>>,
    // msh_18_character_set: Option<Vec<Field<'a>>>,
    // msh_19_principal_language_of_message: Option<Field<'a>>,
    // msh_20_alternate_character_set_handling_scheme: Option<Field<'a>>,
    // msh_21_message_profile_identifier: Option<Vec<Field<'a>>>,
    // msh_22_sending_responsible_organization: Option<Field<'a>>,
    // msh_23_receiving_responsible_organization: Option<Field<'a>>,
    // msh_24_sending_network_address: Option<Field<'a>>,
    // msh_25_receiving_network_address: Option<Field<'a>>,
}

impl<'a> MshSegment<'a> {
    pub fn parse(input: &'a str, delims: &Separators) -> Result<MshSegment<'a>, Hl7ParseError> {
        let fields = input.split(delims.field);

        assert!(fields.next().unwrap() == "MSH");

        let msh = MshSegment {
            fields: Vec::new(),

            msh_1_field_separator: delims.field,
            msh_2_encoding_characters: delims.to_owned(),
            msh_3_sending_application: Field::parse_optional(fields.next(), delims)?,
            msh_4_sending_facility: Field::parse_optional(fields.next(), delims)?,
            msh_5_receiving_application: Field::parse_optional(fields.next(), delims)?,
            msh_6_receiving_facility: Field::parse_optional(fields.next(), delims)?,
            msh_7_date_time_of_message: Field::parse(fields.next()?, delims)?,
        };

        Ok(msh)
    }
}
