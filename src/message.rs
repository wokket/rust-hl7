use super::segments::Segment;
use super::separators::Separators;
use super::*;
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Index;

/// A Message is an entire HL7 message parsed into it's constituent segments, fields, repeats and subcomponents,
/// and it consists of (1 or more) Segments.
/// Message parses the source string into `&str` slices (minimising copying) and can be created using either the [`Message::new()`] function or `TryFrom::try_from()` impl.
/// ## Example:
/// ```
/// # use rusthl7::Hl7ParseError;
/// # use rusthl7::Message;
/// use std::convert::TryFrom;
/// # fn main() -> Result<(), Hl7ParseError> {
/// let source = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|1|Foo\rOBR|2|Bar";
/// let m = Message::new(source); // Note that this method can panic
/// let result = Message::try_from(source); // while try_from() returns a `Result`
/// assert!(result.is_ok());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    source: &'a str,
    pub segments: Vec<Segment<'a>>,
    separators: Separators,
}

impl<'a> Message<'a> {
    /// Takes the source HL7 string and parses it into a message.  Segments
    /// and other data are slices (`&str`) into the source HL7 for minimal (preferably 0) copying.  
    /// ⚠ If an error occurs this method will panic (for back-compat reasons)!  For the preferred non-panicing alternative import the `std::convert::TryFrom` trait and use the `try_from()` function. ⚠
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::Message;
    /// # use std::convert::TryFrom;
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let m = Message::try_from("MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(source: &'a str) -> Message<'a> {
        Message::try_from(source).unwrap()
    }

    /// Queries for segments of the given type (i.e. matches by identifier, or name), returning a set of 0 or more segments.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::Message;
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let source = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|1|Foo\rOBR|2|Bar";
    /// let m = Message::new(source);
    /// let obr_segments = m.segments_by_identifier("OBR")?;
    /// assert_eq!(obr_segments.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn segments_by_identifier(&self, name: &str) -> Result<Vec<&Segment<'a>>, Hl7ParseError> {
        let found: Vec<&Segment<'a>> = self
            .segments
            .iter()
            .filter(|s| s.identifier() == name)
            .collect();
        Ok(found)
    }

    /// Present input vectors of &generics to vectors of &str
    pub fn segments_to_str_vecs(
        segments: Vec<&'a Segment<'a>>,
    ) -> Result<Vec<Vec<&'a str>>, Hl7ParseError> {
        let vecs = segments
            .iter()
            .map(|s| s.fields.iter().map(|f| f.as_str()).collect())
            .collect();

        Ok(vecs)
    }

    /// Returns the source string slice used to create this Message initially.  This method does not allocate.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::Message;
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

    /// Gets the delimiter information for this Message.  
    /// Remember that in HL7 _each individual message_ can have unique characters as separators between fields, repeats, components and sub-components, and so this is a per-message value.
    /// This method does not allocate
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
        let indices = Self::parse_query_string(idx);
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

    /// Parse query/index string to fill-in missing values.
    /// Required when conumer requests "PID.F3.C1" to pass integers down
    /// to the usize indexers at the appropriate positions
    fn parse_query_string(query: &str) -> Vec<&str> {
        fn query_idx_pos(indices: &[&str], idx: &str) -> Option<usize> {
            indices[1..]
                .iter()
                .position(|r| r[0..1].to_uppercase() == idx)
        }
        let indices: Vec<&str> = query.split('.').collect();
        // Leave segment name untouched - complex match
        let mut res = vec![indices[0]];
        // Get segment positions, if any
        let sub_pos = query_idx_pos(&indices, "S");
        let com_pos = query_idx_pos(&indices, "C");
        let rep_pos = query_idx_pos(&indices, "R");
        let fld_pos = query_idx_pos(&indices, "F");
        // Push segment values to result, returning early if possible
        match fld_pos {
            Some(f) => res.push(indices[f + 1]),
            None => {
                // If empty but we have subsections, default to F1
                if rep_pos.is_some() || com_pos.is_some() || sub_pos.is_some() {
                    res.push("F1")
                } else {
                    return res;
                }
            }
        };
        match rep_pos {
            Some(r) => res.push(indices[r + 1]),
            None => {
                // If empty but we have subsections, default to R1
                if com_pos.is_some() || sub_pos.is_some() {
                    res.push("R1")
                } else {
                    return res;
                }
            }
        };
        match com_pos {
            Some(c) => res.push(indices[c + 1]),
            None => {
                // If empty but we have a subcomponent, default to C1
                if sub_pos.is_some() {
                    res.push("C1")
                } else {
                    return res;
                }
            }
        };
        if let Some(s) = sub_pos {
            res.push(indices[s + 1])
        }
        res
    }
}

impl<'a> TryFrom<&'a str> for Message<'a> {
    type Error = Hl7ParseError;

    /// Takes the source HL7 string and parses it into a message.  Segments
    /// and other data are slices (`&str`) into the source HL7 for minimal (preferably 0) copying.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::Message;
    /// # use std::convert::TryFrom;
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let m = Message::try_from("MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4")?;
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        let separators = str::parse::<Separators>(source)?;

        let possible = source
            .split(separators.segment)
            .map(|line| Segment::parse(line, &separators));

        let segments: Vec<Segment> = possible.collect::<Result<Vec<Segment>, Self::Error>>()?;

        let m = Message {
            source,
            segments,
            separators,
        };

        Ok(m)
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
    /// # use rusthl7::Message;
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
        let indices = Self::parse_query_string(&idx);
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
    fn ensure_missing_segments_are_not_found() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;
        assert_eq!(msg.segments_by_identifier("EVN").unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn ensure_segments_convert_to_vectors() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment";
        let msg = Message::try_from(hl7)?;
        let segs = msg.segments_by_identifier("OBR")?;
        let sval = segs.first().unwrap().fields.first().unwrap().as_str();
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
        assert_eq!(msg.query("MSH.F7"), dolly.query("MSH.F7"));
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

    #[test]
    fn ensure_query() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7)?;
        assert_eq!(msg.query("OBR.F1.R1.C2"), "sub&segment");
        assert_eq!(msg.query(&*"OBR.F1.R1.C1".to_string()), "segment"); // Test the Into param with a String
        assert_eq!(msg.query(&*String::from("OBR.F1.R1.C1")), "segment");
        assert_eq!(msg.query("MSH.F1"), "^~\\&");
        Ok(())
    }

    #[cfg(feature = "string_index")]
    mod string_index_tests {
        use super::*;
        #[test]
        fn ensure_index() -> Result<(), Hl7ParseError> {
            let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
            let msg = Message::try_from(hl7)?;
            assert_eq!(msg["OBR.F1.R1.C2"], "sub&segment");
            assert_eq!(msg[&*"OBR.F1.R1.C1".to_string()], "segment"); // Test the Into param with a String
            assert_eq!(msg[String::from("OBR.F1.R1.C1")], "segment");
            assert_eq!(msg[String::from("OBR.F1.C1")], "segment"); // Test missing element in selector
            assert_eq!(msg[String::from("OBR.F1.R1.C2.S1")], "sub");
            println!("{}", Message::parse_query_string("MSH.F2").join("."));
            assert_eq!(msg["MSH.F2"], "^~\\&");
            Ok(())
        }
    }
}
