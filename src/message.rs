use super::segments::{Segment, MshSegment};
use super::separators::Separators;
use super::*;
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Index;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponents,
/// and it consists of (1 or more) Segments.
/// Message parses the source string into &str slices (minimising copying)
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub source: &'a str,
    pub segments: Vec<Segment<'a>>,
    separators: Separators
}

impl<'a> Message<'a> {
    pub fn new(source: &'a str) -> Message<'a> {
        let delimiters = str::parse::<Separators>(source);
        let delimiters = delimiters.unwrap();
        let segments: Vec<Segment<'a>> = source
            .split(delimiters.segment)
            .map(|line| Segment::parse(line, &delimiters).unwrap())
            .collect();

        Message {
            source,
            segments: segments,
            separators: delimiters
        }
    }
    /// Extracts header element for external use
    pub fn msh(&self) -> Result<MshSegment, Hl7ParseError> {
        let seg = self
            .segments
            .iter()
            .find(|s| s.fields[0].source == "MSH" ).unwrap();
        let segment = MshSegment::parse(seg.source, &self.separators)
            .expect("Failed to parse MSH segment");
        Ok(segment)
    }

    /// Extracts generic elements for external use by matching first field to name
    pub fn segments_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<&Segment<'a>>, Hl7ParseError> {
        let found: Vec<&Segment<'a>> = self
            .segments
            .iter()
            .filter(|s| s.fields[0].source == name )
            .collect();
        Ok(found)
    }

    /// Present input vectors of &generics to vectors of &str
    pub fn segments_to_str_vecs(
        segments: Vec<&Segment<'a>>,
    ) -> Result<Vec<Vec<&'a str>>, Hl7ParseError> {
        let vecs = segments
            .iter()
            .map(|s| s.fields.iter().map(|f| f.value()).collect())
            .collect();
        Ok(vecs)
    }

    /// Returns the source string slice used to create this Message initially.  This method does not allocate.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::message::Message;
    /// # use std::convert::TryFrom;
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let source = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4";
    /// let m = Message::try_from(source)?;
    /// assert_eq!(source, m.as_str());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.source
    }

    /// Gets the delimiter information for this Message
    pub fn get_separators(&self) -> Separators {
        self.separators
    }

    /// Access Segment, Field, or sub-field string references by string index
    pub fn query<'b, S>(&self, idx: S) -> &'a str
    where
        S: Into<&'b str>,
    {
        let idx = idx.into();

        // Parse index elements
        let indices: Vec<&str> = idx.split('.').collect();
        let seg_name = indices[0];
        // Find our first segment without offending the borow checker
        let seg_index = self
            .segments
            .iter()
            .position(|r| &r.as_str()[..seg_name.len()] == seg_name)
            .expect("Segment not found");
        let seg = &self.segments[seg_index];
        if indices.len() < 2 {
            seg.source
        } else {
            let query = indices[1..].join(".");
            seg.query(&*query)
        }
    }
}

impl<'a> TryFrom<&'a str> for Message<'a> {
    type Error = Hl7ParseError;

    /// Takes the source HL7 string and parses it into this message.  Segments
    /// and other data are slices (`&str`) into the source HL7
    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        let delimiters = str::parse::<Separators>(source)?;

        let segments: Result<Vec<Segment<'a>>, Hl7ParseError> = source
            .split(delimiters.segment)
            .map(|line| Segment::parse(line, &delimiters))
            .collect();

        let msg = Message {
            source,
            segments: segments?,
            separators: delimiters
        };

        Ok(msg)
    }
}

impl<'a> Display for Message<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl<'a> Clone for Message<'a> {
    /// Creates a new cloned Message object referencing the same source slice as the original.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::message::Message;
    /// # use std::convert::TryFrom;
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let m = Message::try_from("MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4")?;
    /// let cloned = m.clone(); // this object is looking at the same string slice as m
    /// # Ok(())
    /// # }
    /// ```
    fn clone(&self) -> Self {
        Message::try_from(self.source).unwrap()
    }
}

impl<'a> Index<usize> for Message<'a> {
    type Output = &'a str;

    /// Access Segment string reference by numeric index
    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.segments.len() {
            return &"";
        }
        &self.segments[idx].source
    }
}
#[cfg(feature = "string_index")]
impl<'a> Index<String> for Message<'a> {
    type Output = &'a str;

    /// Access Segment, Field, or sub-field string references by string index
    #[cfg(feature = "string_index")]
    fn index(&self, idx: String) -> &Self::Output {
        // Parse index elements
        let indices: Vec<&str> = idx.split('.').collect();
        let seg_name = indices[0];
        // Find our first segment without offending the borow checker
        let seg_index = self
            .segments
            .iter()
            .position(|r| &r.as_str()[..seg_name.len()] == seg_name)
            .expect("Segment not found");
        let seg = &self.segments[seg_index];
        if indices.len() < 2 {
            &seg.source
        } else {
            &seg[indices[1..].join(".")]
        }
    }
}

#[cfg(feature = "string_index")]
impl<'a> Index<&str> for Message<'a> {
    type Output = &'a str;

    #[cfg(feature = "string_index")]
    fn index(&self, idx: &str) -> &Self::Output {
        &self[String::from(idx)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_segments_are_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;

        assert_eq!(msg.segments.len(), 2);
        Ok(())
    }

    #[test]
    fn ensure_segments_are_found() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;

        assert_eq!(msg.segments_by_name("OBR").unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn ensure_msh_is_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;

        assert_eq!(msg.msh().unwrap().msh_1_field_separator, '|');
        Ok(())
    }
    #[test]
    fn ensure_segments_convert_to_vectors() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;
        let segs = msg.segments_by_name("OBR")?;
        let sval = segs.first().unwrap().fields.first().unwrap().value();
        let vecs = Message::segments_to_str_vecs(segs).unwrap();
        let vval = vecs.first().unwrap().first().unwrap();

        assert_eq!(vval, &sval);
        Ok(())
    }
    #[test]
    fn ensure_clones_are_owned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;
        // Verify that we can clone and take ownership
        let dolly = msg.clone();
        let dolly = dolly.to_owned();
        assert_eq!(
            msg.msh().unwrap().msh_7_date_time_of_message,
            dolly.msh().unwrap().msh_7_date_time_of_message
        );
        Ok(())
    }

    #[test]
    fn ensure_to_string() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;
        assert_eq!(msg.to_string(), String::from(hl7));
        Ok(())
    }

    #[test]
    fn ensure_message_creation() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg0 = Message::try_from(hl7)?;
        let msg1 = Message::new(hl7);

        assert_eq!(msg0, msg1);
        Ok(())
    }

    #[cfg(feature = "string_index")]
    mod string_index_tests {
        use super::*;
        #[test]
        fn ensure_index() -> Result<(), Hl7ParseError> {
            let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
            let msg = Message::try_from(hl7)?;
            assert_eq!(msg.query("OBR.F1.R2.C1"), "sub");
            assert_eq!(msg.query(&*"OBR.F1.R2.C1".to_string()), "sub"); // Test the Into param with a String
            assert_eq!(msg[String::from("OBR.F1.R2.C1")], "sub");
            assert_eq!(msg["MSH.F3"], "ELAB-3");
            Ok(())
        }
    }
}
