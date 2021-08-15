/*!

# RustHl7 - A HL7 V2 message parser

This crate is attempting to provide a fully spec-compliant HL7 V2 message parser.  Note that _interpreting_ the parsed message elements into a strongly
typed segment/message format is specifically **out of scope** as there's simply too many variants over too many versions for me to go there (maybe 
someone else could code-gen a crate using this this crate to provide the source information?).  



## Crate Features
The default use of this crate should be as close to spec-compliant as we can manage.  If there's features you absolutely are sure you don't need (because, say, you control both ends of the wire)
you can run with `default-features = false` and then opt back in to the features you _do_ need.

- `escape-sequence`: This feature includes the decoding of [`EscapeSequence`] in the values returned from the `query` functions

*/

pub mod escape_sequence;
pub mod fields;
pub mod message;
pub mod segments;
pub mod separators;

#[derive(Debug, thiserror::Error)]
pub enum Hl7ParseError {
    #[error("Unexpected error: {0}")]
    Generic(String),

    #[error("Failure parsing MSH1/MSH2 while discovering separator chars: {0}")]
    Msh1Msh2(String),

    #[error("Required value missing")]
    MissingRequiredValue(),
}
