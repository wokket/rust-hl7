/*!

# RustHl7 - A HL7 V2 message parser and library

This crate is attempting to provide the tooling for a fully spec-compliant HL7 V2 message parser.  Note that _interpreting_ the parsed message elements into a strongly
typed segment/message format is specifically **out of scope** as there's simply too many variants over too many versions for me to go there (maybe
someone else could code-gen a crate using this this crate to provide the source information?).

This crate tries to provide the tools to build HL7 systems without dictating _how_ to build your system, there's no such thing as one-size-fits all in healthcare!

*/

pub mod escape_sequence;
pub mod fields;
pub mod message;
pub mod segments;
pub mod selector;
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
