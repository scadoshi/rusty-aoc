use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseU12Error {
    #[error("must fit into 12 bits")]
    Overflow,
}
