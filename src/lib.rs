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
