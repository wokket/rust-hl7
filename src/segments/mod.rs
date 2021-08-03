pub mod generic;
pub mod msh;

use super::fields::Field;
use super::separators::Separators;
use super::*;
use generic::GenericSegment;
use msh::MshSegment;

/// A single segment, 0x13 delimited line from a source HL7 message consisting of multiple fields.
#[derive(Debug, PartialEq)]
pub enum Segment<'a> {
    MSH(MshSegment<'a>),
    Generic(GenericSegment<'a>),
}

impl<'a> Segment<'a> {
    /// Convert the given line of text into a Segment.
    pub fn parse(input: &'a str, delims: &Separators) -> Result<Segment<'a>, Hl7ParseError> {
        let fields: Result<Vec<Field<'a>>, Hl7ParseError> = input
            .split(delims.field)
            .map(|line| Field::parse(line, &delims))
            .collect();

        let fields = fields?;

        let seg = match fields[0].value() {
            "MSH" => Segment::MSH(MshSegment::parse(&input, delims)?),
            _ => Segment::Generic(GenericSegment {source: &input, delim: delims.field, fields }),
        };

        Ok(seg)
    }

    /// Export source to owned String
    pub fn to_string(&self) -> String {
        match self {
            Segment::MSH(m) => m.to_string(),
            Segment::Generic(g) => g.to_string()
        }
    }

    /// Export source to str
    pub fn as_str(&self) -> &'a str {
        match self {
            Segment::MSH(m) => m.as_str(),
            Segment::Generic(g) => g.as_str(),
        }
    }
}

// impl<'a> Index<String> for Segment<'a> {
//     type Output = &str;
//     // Handle 2nd element of `PID.F3.R1.C1`
//     fn index(&self, idx: String) -> &Self::Output {
//         let slices: Vec<&str> = idx.split(".");
//         let fieldno = slices.first()[1..] as u8;
//         let found: Vec<&str> = self
//             .fields
//             .iter()
//             .filter_map(|s| match s {
//                 Segment::Generic((x) => {
//                     if x.fields[fieldno] {
//                         Some(x)
//                     } else {
//                         None
//                     }
//                 }
//                 _ => None,
//             })
//             .map(|s| 
//                 if slices.len() > 1 {
//                     s.index(slices[1..]).join(".")
//                 } else {
//                     s.as_str()
//                 }
//             )
//             .collect();
//         found
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_fields_are_added() -> Result<(), Hl7ParseError> {
        let hl7 = "SEG|field 1|field 2|field 3";
        let delims = Separators::default();

        if let Segment::Generic(seg) = Segment::parse(hl7, &delims)? {
            assert_eq!(seg.fields.len(), 4);
        } else {
            assert!(false);
        }
        Ok(())
    }

    #[test]
    fn ensure_msh_is_returned() -> Result<(), Hl7ParseError> {
        let hl7 = "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4";
        let delims = Separators::default();

        if let Segment::MSH(_) = Segment::parse(hl7, &delims)? {
            //all good, fall through to ok
        } else {
            assert!(false);
        }
        Ok(())
    }
}
