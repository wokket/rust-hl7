/*!
The Selector functionality provides the ability to query into a HL7 message and select individual values using a path notation.

## Example
```
# use rusthl7::message::Message;
# fn main() -> Result<(), Hl7ParseError> {
    var msg =  Message::try_from("MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4")?;
    var selector = Selector::new("MSH.F3"); // MSH Segment, Field 3 (Sending Application)
    let val = selector.query(msg);
    assert_eq!(val, "GHH LAB");
    # Ok(())
# }
```


*/
use super::*;

pub struct Selector{

}

