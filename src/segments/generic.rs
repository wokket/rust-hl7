use super::fields::Field;
use super::separators::Separators;
use super::*;
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
    pub fn parse(input: &'a str, delims: &Separators) -> Result<GenericSegment<'a>, Hl7ParseError> {
        let fields: Result<Vec<Field<'a>>, Hl7ParseError> = input
            .split(delims.field)
            .map(|line| Field::parse(line, &delims))
            .collect();

        let fields = fields?;
        let seg = GenericSegment {
            source: &input,
            delim: delims.segment,
            fields
        };
        Ok(seg)
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

impl<'a> Index<usize> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    fn index(&self, fidx: usize) -> &Self::Output {
        if fidx > self.fields.len() - 1 { return &"" };
        &self.fields[fidx].source
    }
}

impl<'a> Index<(usize, usize)> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field component as string reference
    fn index(&self, fidx: (usize, usize)) -> &Self::Output {
        if fidx.0 > self.fields.len() - 1 ||
        fidx.1 > self.fields[fidx.0].components.len() - 1 {
            return &""
        }
        &self.fields[fidx.0][fidx.1]
    }
}

impl<'a> Index<(usize, usize, usize)> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field subcomponent as string reference
    fn index(&self, fidx: (usize, usize, usize)) -> &Self::Output {
        if fidx.0 > self.fields.len() - 1 ||
        fidx.1 > self.fields[fidx.0].components.len() - 1 ||
        fidx.2 > self.fields[fidx.0].subcomponents[fidx.1].len() - 1 {
            return &""
        }
        &self.fields[fidx.0][(fidx.1, fidx.2)]
    }
}
impl<'a> Index<String> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field as string reference
    fn index(&self, fidx: String) -> &Self::Output {
        let sections = fidx.split(".").collect::<Vec<&str>>();
        match sections.len() {
            1 => {
                let stringnum = sections[0].chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>();
                let idx: usize = stringnum.parse().unwrap();
                &self[idx]
            },
            _ => {
                let stringnum = sections[0].chars()
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

    /// Access Segment, Field, or sub-field string references by string index
    fn index(&self, idx: &str) -> &Self::Output {
        &self[String::from(idx)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::message::Message;
    use super::super::*;

    #[test]
    fn ensure_numeric_index() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::from_str(hl7).unwrap();
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
        let msg = Message::from_str(hl7).unwrap();
        let (f, c, s, oob) = match &msg.segments[1] {
            Segment::Generic(x) => (
                x[String::from("F1")], 
                x[String::from("F1.R2")], 
                x[String::from("F1.R2.C1")],
                String::from(x[String::from("F10")]) +
                    x[String::from("F1.R10")] +
                    x[String::from("F1.R2.C10")]
            ),
            _ => ("", "", "", String::from("")),
        };
        assert_eq!(f, "segment^sub&segment");
        assert_eq!(c, "sub&segment");
        assert_eq!(s, "sub");
        assert_eq!(oob, "");
    }
}
