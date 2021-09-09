/*!
 A short example demonstrating one way to use this library for HL7 processing.
*/

use rusthl7::{EscapeSequence, Message};
use std::{convert::TryFrom, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // Normally message would come over the wire from a remote service etc.
    // Consider using the hl7-mllp-codec crate or similar to make building those network services easier.
    let hl7_string = get_sample_message();

    // Parse the string into a structured entity
    let message = Message::try_from(hl7_string)?;

    // We can deep query message fields using the `query` functionality
    let postcode = message.query("PID.F11.C5"); // Field 11, Component 5
    assert_eq!(postcode, "35292");

    // If you have the potential for escape sequences in your data you can process those using `EscapeSequence`
    let charge_to_practice = message.query("OBR.F23");
    assert_eq!(charge_to_practice, r#"Joes Obs \T\ Gynae"#);

    let decoder = EscapeSequence::new(message.get_separators());
    let charge_to_practice = decoder.decode(charge_to_practice); // Handle the escape sequences
    assert_eq!(charge_to_practice, "Joes Obs & Gynae"); // converted the \T\ sequence to an ampersand

    Ok(())
}

fn get_sample_message() -> &'static str {
    "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||Joes Obs \\T\\ Gynae||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F"
}
