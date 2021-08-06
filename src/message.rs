use super::segments::*;
use super::separators::Separators;
use super::*;
use std::ops::Index;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponents,
/// and it consists of (1 or more) Segments.
/// Message parses the source string into &str slices (minimising copying)
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub source: &'a str,
    pub segments: Vec<Segment<'a>>,
}

impl<'a> Message<'a> {
    /// Takes the source HL7 string and parses it into this message.  Segments
    /// and other data are slices (`&str`) into the source HL7
    pub fn from_str(source: &'a str) -> Result<Self, Hl7ParseError> {
        let delimiters = str::parse::<Separators>(source)?;

        let segments: Result<Vec<Segment<'a>>, Hl7ParseError> = source
            .split(delimiters.segment)
            .map(|line| Segment::parse(line, &delimiters))
            .collect();

        let msg = Message {
            source,
            segments: segments?,
        };

        Ok(msg)
    }

    /// Extracts header element for external use
    pub fn msh(&self) -> Result<&msh::MshSegment, Hl7ParseError> {
        let segment = self
            .segments
            .iter()
            .find_map(|s| match s {
                segments::Segment::MSH(x) => Some(x),
                _ => None,
            })
            .expect("Failed to find hl7 header");
        Ok(segment)
    }

    /// Extracts all generic elements for external use
    pub fn generic_segments(&self) -> Result<Vec<&generic::GenericSegment>, Hl7ParseError> {
        let generics: Vec<&generic::GenericSegment> = self
            .segments
            .iter()
            .filter_map(|s| match s {
                segments::Segment::Generic(x) => Some(x),
                _ => None,
            })
            .collect();
        Ok(generics)
    }

    /// Extracts generic elements for external use by matching first field to name
    pub fn generic_segments_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<&generic::GenericSegment>, Hl7ParseError> {
        let found: Vec<&generic::GenericSegment> = self
            .segments
            .iter()
            .filter_map(|s| match s {
                segments::Segment::Generic(x) => {
                    if x.fields.first().unwrap().value() == name {
                        Some(x)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();
        Ok(found)
    }

    /// Present input vectors of &generics to vectors of &str
    pub fn segments_to_str_vecs(
        segments: Vec<&generic::GenericSegment<'a>>,
    ) -> Result<Vec<Vec<&'a str>>, Hl7ParseError> {
        let vecs = segments
            .iter()
            .map(|s| s.fields.iter().map(|f| f.value()).collect())
            .collect();
        Ok(vecs)
    }

    /// Export source to owned String
    pub fn to_string(&self) -> String {
        self.source.clone().to_owned()
    }

    /// Export source to str
    pub fn as_str(&self) -> &'a str {
        self.source
    }
}

impl<'a> Clone for Message<'a> {
    /// Creates a new Message object using a clone of the original's source
    fn clone(&self) -> Self {
        let msg = Message::from_str(self.source.clone()).unwrap();
        msg
    }
}

impl<'a> Index<usize> for Message<'a> {
    type Output = &'a str;

    /// Access Segment string reference by numeric index
    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.segments.len() {
            return &""
        }
        let seg = &self.segments[idx];
        // Return the appropriate source reference
        match seg {
            // Short circuit for now
            Segment::MSH(m) => &m.source,
            // Parse out slice depth
            Segment::Generic(g) => &g.source,
        }
    }
}

impl<'a> Index<String> for Message<'a> {
    type Output = &'a str;

    /// Access Segment, Field, or sub-field string references by string index
    fn index(&self, idx: String) -> &Self::Output {
        // Parse index elements
        let indices: Vec<&str> = idx.split(".").collect();
        let seg_name = indices[0];
        // Find our first segment without offending the borow checker
        let seg_index = self
            .segments
            .iter()
            .position(|r| &r.as_str()[..seg_name.len()] == seg_name);
        match seg_index {
            Some(_) => {},
            None => return &""
        }
        let seg = &self.segments[seg_index.unwrap()];
        // Return the appropriate source reference
        match seg {
            // Short circuit for now
            Segment::MSH(m) => &m.source,
            // Parse out slice depth
            Segment::Generic(g) => if indices.len() < 2 {
                &g.source
            } else {
                &g[indices[1..].join(".")]
            }
        }
    }
}

impl<'a> Index<&str> for Message<'a> {
    type Output = &'a str;

    /// Access Segment, Field, or sub-field string references by string index
    fn index(&self, idx: &str) -> &Self::Output {
        &self[String::from(idx)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_segments_are_added() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.segments.len(), 2);
        Ok(())
    }

    #[test]
    fn ensure_generic_segments_are_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.generic_segments().unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn ensure_generic_segments_are_found() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.generic_segments_by_name("OBR").unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn ensure_msh_is_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;

        assert_eq!(msg.msh().unwrap().msh_1_field_separator, '|');
        Ok(())
    }
    #[test]
    fn ensure_segments_convert_to_vectors() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;
        let segs = msg.generic_segments_by_name("OBR")?;
        let sval = segs.first().unwrap().fields.first().unwrap().value();
        let vecs = Message::segments_to_str_vecs(segs).unwrap();
        let vval = vecs.first().unwrap().first().unwrap();

        assert_eq!(vval, &sval);
        Ok(())
    }
    #[test]
    fn ensure_clones_are_owned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::from_str(hl7)?;
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
        let msg = Message::from_str(hl7)?;
        assert_eq!(msg.to_string(), String::from(hl7));
        Ok(())
    }

    #[test]
    fn ensure_index() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::from_str(hl7)?;
        assert_eq!(msg[String::from("OBR.F1.R2.C1")], "sub");
        Ok(())
    }
}
