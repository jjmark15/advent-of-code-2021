#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("could not parse input")]
pub(crate) struct ParseInputError;
