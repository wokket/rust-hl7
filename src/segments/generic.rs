use super::fields::Field;
use std::ops::Index;

/// A generic bag o' fields, representing an arbitrary segment.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericSegment<'a> {
    pub source: &'a str,
    pub delim: char,
    pub fields: Vec<Field<'a>>,
}

impl<'a> GenericSegment<'a> {
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
        &self.fields[fidx].source
    }
}

impl<'a> Index<(usize,usize)> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field component as string reference
    fn index(&self, fidx: (usize,usize)) -> &Self::Output {
        &self.fields[fidx.0][fidx.1]
    }
}

impl<'a> Index<(usize,usize,usize)> for GenericSegment<'a> {
    type Output = &'a str;
    /// Access Field subcomponent as string reference
    fn index(&self, fidx: (usize,usize,usize)) -> &Self::Output {
        &self.fields[fidx.0][(fidx.1,fidx.2)]
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::super::message::Message;

    #[test]
    fn ensure_index() {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rOBR|segment^sub&segment";
        let msg = Message::from_str(hl7).unwrap();
        let (f,c,s) = match &msg.segments[1] {
            Segment::Generic(x) => (
                x[1], x[(1,2)], x[(1,2,1)]
            ),
            _ => ("", "", "")
        };
        assert_eq!(f,"segment^sub&segment");
        assert_eq!(c,"sub&segment");
        assert_eq!(s,"sub");
    }
}