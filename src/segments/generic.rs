use super::{fields::Field, separators::Separators, Hl7ParseError};
use std::fmt::Display;
use std::ops::Index;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericSegment<'a> {
    pub source: &'a str,
    pub delim: char,
    pub fields: Vec<Field<'a>>,
}

impl<'a> GenericSegment<'a> {
    /// Convert the given line of text into a GenericSegment.
    pub fn parse<S: Into<&'a str>>(
        input: S,
        delims: &Separators,
    ) -> Result<GenericSegment<'a>, Hl7ParseError> {
        let input = input.into();

        let fields: Result<Vec<Field<'a>>, Hl7ParseError> = input
            .split(delims.field)
            .map(|line| Field::parse(line, delims))
            .collect();

        let fields = fields?;
        let seg = GenericSegment {
            source: input,
            delim: delims.segment,
            fields,
        };
        Ok(seg)
    }

    /// Export source to str
    #[inline]
    pub fn as_str(&self) -> &'a str {
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
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                self[idx]
            }
            _ => {
                let stringnum = sections[0]
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                let field = &self.fields[idx];
                let query = sections[1..].join(".");

                field.query(&*query)
            }
        }
    }
}

impl<'a> Display for GenericSegment<'a> {
    /// Required for to_string() and other formatter consumers
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl<'a> Index<usize> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    fn index(&self, fidx: usize) -> &Self::Output {
        if fidx > self.fields.len() - 1 {
            return &"";
        };
        &self.fields[fidx].source
    }
}

impl<'a> Index<(usize, usize)> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field component as string reference
    fn index(&self, fidx: (usize, usize)) -> &Self::Output {
        if fidx.0 > self.fields.len() - 1 || fidx.1 > self.fields[fidx.0].components.len() - 1 {
            return &"";
        }
        &self.fields[fidx.0][fidx.1]
    }
}

impl<'a> Index<(usize, usize, usize)> for GenericSegment<'a> {
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

impl<'a> Index<String> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    fn index(&self, fidx: String) -> &Self::Output {
        let sections = fidx.split('.').collect::<Vec<&str>>();
        match sections.len() {
            1 => {
                let stringnum = sections[0]
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                &self[idx]
            }
            _ => {
                let stringnum = sections[0]
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                &self.fields[idx][sections[1..].join(".")]
            }
        }
    }
}

impl<'a> Index<&str> for GenericSegment<'a> {
    type Output = &'a str;

    /// DEPRECATED.  Access Segment, Field, or sub-field string references by string index
    #[allow(useless_deprecated)]
    #[deprecated(note = "This will be removed in a future version")]
    fn index(&self, idx: &str) -> &Self::Output {
        &self[String::from(idx)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::message::Message;
    use super::super::*;
    use std::convert::TryFrom;

    #[test]
    fn ensure_numeric_index() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7).unwrap();
        let (f, c, s) = match &msg.segments[1] {
            Segment::Generic(x) => (x[1], x[(1, 1)], x[(1, 1, 0)]),
            _ => ("", "", ""),
        };
        assert_eq!(f, "segment^sub&segment");
        assert_eq!(c, "sub&segment");
        assert_eq!(s, "sub");
    }

    #[test]
    fn ensure_string_index() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::try_from(hl7).unwrap();
        let (f, c, s, oob) = match &msg.segments[1] {
            Segment::Generic(x) => (
                x.query("F1"),                       //&str
                x.query("F1.R2"),                    // &str
                x.query(&*String::from("F1.R2.C1")), //String
                String::from(x.query("F10")) + x.query("F1.R10") + x.query("F1.R2.C10"),
            ),
            _ => ("", "", "", String::from("")),
        };
        assert_eq!(f, "segment^sub&segment");
        assert_eq!(c, "sub&segment");
        assert_eq!(s, "sub");
        assert_eq!(oob, "");
    }
}
