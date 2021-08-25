/*!
 A short example demonstrating one way to use this library for HL7 processing.
*/

use rusthl7::{fields::Field, message::Message, separators::Separators,Hl7ParseError};
use std::{convert::TryFrom, error::Error, fmt::Display};

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
}
/// Common formatter trait implementation for the strongly-typed segment
impl<'a> Display for MshSegment<'a> {
    /// Required for to_string() and other formatter consumers
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}
/// Common clone trait implementation for the strongly-typed segment
impl<'a> Clone for MshSegment<'a> {
    /// Creates a new Message object using _the same source_ slice as the original.
    fn clone(&self) -> Self {
        let delims = self.msh_2_encoding_characters;
        MshSegment::parse(self.source, &delims).unwrap()
    }
}

/// Extracts header element for external use
pub fn msh<'a>(msg: &Message<'a>) -> Result<MshSegment<'a>, Hl7ParseError> {
    let seg = msg.segments_by_name("MSH").unwrap()[0];
    let segment =
        MshSegment::parse(seg.source, &msg.get_separators()).expect("Failed to parse MSH segment");
    Ok(segment)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Normally message would come over the wire from a remote service etc.
    // Consider using the hl7-mllp-codec crate or similar to make building those network services easier.
    let hl7_string = get_sample_message();

    // Parse the string into a structured entity
    let message = Message::try_from(hl7_string)?;

    // Get a strongly-typed segment from generic data
    let header = msh(&message).expect("Failed to extract MSH");
    let send_fac = header.msh_4_sending_facility.unwrap().source;
    assert_eq!(send_fac, message.segments[0].fields[3].source);

    Ok(())
}

fn get_sample_message() -> &'static str {
    "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||Joes Obs \\T\\ Gynae||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F"
}
