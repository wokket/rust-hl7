/*!
The Selector functionality provides the ability to query into a HL7 message and select individual values using a path notation.

## Example
```
# use rusthl7::Hl7ParseError;
# use rusthl7::message::Message;
# use rusthl7::selector;
# use std::convert::TryFrom;
# fn main() -> Result<(), Hl7ParseError> {
    let msg =  Message::try_from("MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4")?;
    let val = selector::query(&msg, "MSH.F2"); // MSH Segment, Field 2
    assert_eq!(val, "GHH LAB");
    # Ok(())
# }
```

*/
use super::message::Message;

pub fn query<'a, 'query, S>(msg: &'a Message, path: S) -> &'a str
where
    S: Into<&'query str>,
{
    query_message(msg, path)
}

/// Access Segment, Field, or sub-field string references by string index
pub fn query_message<'a, 'query, S>(msg: &'a Message, path: S) -> &'a str
where
    S: Into<&'query str>,
{
    let path = path.into();

    // Parse index elements
    let indices: Vec<&str> = path.split('.').collect();
    let seg_name = indices[0];
    // Find our first segment without offending the borow checker
    let seg_index = msg
        .segments
        .iter()
        .position(|r| &r.as_str()[..seg_name.len()] == seg_name)
        .expect("Segment not found");
    let seg = &msg.segments[seg_index];
    if indices.len() < 2 {
        seg.source
    } else {
        let query = indices[1..].join(".");
        seg.query(&*query)
    }
}

#[cfg(test)]
mod tests {
    use crate::message::Message;
    use crate::selector;
    use crate::Hl7ParseError;
    use std::convert::TryFrom;

    #[test]
    fn ensure_message_query() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7)?;

        assert_eq!(selector::query(&msg, "OBR.F1.R1.C2"), "sub&segment");
        // Test the Into param with a String
        assert_eq!(
            selector::query(&msg, &*"OBR.F1.R1.C1".to_string()),
            "segment"
        );
        assert_eq!(
            selector::query(&msg, &*String::from("OBR.F1.R1.C1")),
            "segment"
        );
        assert_eq!(selector::query(&msg, "MSH.F1"), "^~\\&");
        Ok(())
    }
}
