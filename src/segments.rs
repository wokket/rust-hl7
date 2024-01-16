use crate::{Field, Hl7ParseError, Separators};
use std::fmt::Display;
use std::ops::Index;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq, Clone)]
pub struct Segment<'a> {
    pub source: &'a str,
    delim: char,
    pub fields: Vec<Field<'a>>,
}

impl<'a> Segment<'a> {
    /// Convert the given line of text into a Segment.  NOTE: This is not normally needed to be called directly by
    /// consumers but is used indirectly via [`crate::Message::new()`].
    pub fn parse<S: Into<&'a str>>(
        input: S,
        delims: &Separators,
    ) -> Result<Segment<'a>, Hl7ParseError> {
        // non-generic inner to reduce compile times/code bloat
        fn inner<'a>(input: &'a str, delims: &Separators) -> Result<Segment<'a>, Hl7ParseError> {
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

        let input = input.into();
        inner(input, delims)
    }

    /// Get the identifier (ie type, or name) for this segment.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::{Segment, Separators};
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let segment = Segment::parse("OBR|field1|field2", &Separators::default())?;
    /// assert_eq!("OBR", segment.identifier());
    /// # Ok(())
    /// # }
    /// ```
    /// eg a segment `EVN||200708181123||` has an identifer of `EVN`.
    pub fn identifier(&self) -> &'a str {
        self.fields[0].source
    }

    /// Returns the original `&str` used to initialise this Segment.  This method does not allocate.
    /// ## Example:
    /// ```
    /// # use rusthl7::Hl7ParseError;
    /// # use rusthl7::Message;
    /// # use std::convert::TryFrom;
    /// # fn main() -> Result<(), Hl7ParseError> {
    /// let source = "MSH|^~\\&|GHH LAB|ELAB-3\rOBR|field1|field2";
    /// let m = Message::try_from(source)?;
    /// let obr = &m.segments[1];
    /// assert_eq!("OBR|field1|field2", obr.as_str());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_str(&'a self) -> &'a str {
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
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                self[idx]
            }
            _ => {
                let stringnum = sections[0]
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                if idx > self.fields.len() - 1 {
                    return "";
                }
                let field = &self.fields[idx];
                let query = sections[1..].join(".");

                field.query(&*query)
            }
        }
    }
}

impl<'a> Display for Segment<'a> {
    /// Required for to_string() and other formatter consumers.  This returns the source string that represents the segment.
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
impl<'a> Index<&str> for Segment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    fn index(&self, fidx: &str) -> &Self::Output {
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
                // return &&self.source[3..3]; //TODO figure out how to return a string ref safely
                return &"|";
            } else {
                idx = idx - 1
            }
        }
        match sections.len() {
            1 => &self[idx],
            _ => {
                if idx < self.fields.len() {
                    &self.fields[idx][sections[1..].join(".")]
                } else {
                    &""
                }
            }
        }
    }
}

#[cfg(feature = "string_index")]
impl<'a> Index<String> for Segment<'a> {
    type Output = &'a str;

    /// Access Segment, Field, or sub-field string references by string index
    fn index(&self, idx: String) -> &Self::Output {
        &self[idx.as_str()]
    }
}

#[cfg(test)]
mod tests {
    use crate::Message;
    use std::convert::TryFrom;

    #[test]
    fn ensure_numeric_index() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7).unwrap();
        let x = &msg.segments[1];
        let (f, c, s) = (x[1], x[(1, 0)], x[(1, 0, 1)]);
        assert_eq!(f, "segment^sub&segment");
        assert_eq!(c, f);
        assert_eq!(s, "sub&segment");
    }

    #[test]
    fn ensure_string_query() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7).unwrap();
        let x = &msg.segments[1];
        let (f, c, s, oob) = (
            x.query("F1"),                       //&str
            x.query("F1.R1"),                    // &str
            x.query(&*String::from("F1.R1.C1")), //String
            String::from(x.query("F10")) + x.query("F1.R10") + x.query("F1.R2.C10"),
        );
        assert_eq!(f, "segment^sub&segment");
        assert_eq!(c, f);
        assert_eq!(s, "segment");
        assert_eq!(oob, "");
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
                x["F1"],                  // &str
                x["F1.R1"],               // &str
                x["F1.R1.C1".to_owned()], // String
                x["F1.R2.C2"],
            );
            assert_eq!(f, "segment^sub&segment");
            assert_eq!(c, "segment^sub&segment");
            assert_eq!(s, "segment");
            assert_eq!(oob, "");
        }
    }
}
