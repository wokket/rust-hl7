use super::fields::Field;
use super::separators::Separators;
use super::*;

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
    pub fn parse(input: &'a str, delims: &Separators) -> Result<MshSegment<'a>, Hl7ParseError> {
        let mut fields = input.split(delims.field);

        assert!(fields.next().unwrap() == "MSH");

        let _ = fields.next(); //consume the delimiter chars

        let msh = MshSegment {
            source: &input,
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
    pub fn as_str(&self) -> &'a str {
        self.source
    }

    /// Present MSH data in Generic segment
    pub fn as_generic(&self) -> Result<GenericSegment<'a>, Hl7ParseError> {
        let delims = self.msh_2_encoding_characters;
        GenericSegment::parse(self.source, &delims)
    }
}

use std::fmt::Display;
impl<'a> Display for MshSegment<'a> {
    /// Required for to_string() and other formatter consumers
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl<'a> Clone for MshSegment<'a> {
    /// Creates a new Message object using a clone of the original's source
    fn clone(&self) -> Self {
        let delims = self.msh_2_encoding_characters;
        MshSegment::parse(self.source, &delims).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn ensure_msh_converts_to_generic() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4";
        let delims = Separators::default();

        let msh = MshSegment::parse(hl7, &delims)?;
        let gen = msh.as_generic().unwrap();
        assert_eq!("ELAB-3",gen["F3"]);
        Ok(())
    }

    #[test]
    fn ensure_msh_clones_correctly() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4";
        let delims = Separators::default();

        let msh = MshSegment::parse(hl7, &delims)?;
        let dolly = msh.clone();
        assert_eq!(msh,dolly);
        Ok(())
    }
}
