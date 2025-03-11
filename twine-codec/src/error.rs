use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum TwineCodecError {
    #[error("Could not convert buffer of bytes to {0}")]
    BufferBytesConversion(&'static str),

    #[error("Number of bytes exceeds buffer maximum length. {0} expected {1}, found {2}")]
    BufferMaxLength(&'static str, usize, usize),

    #[error("Could not build {0}")]
    TypeBuildError(&'static str),

    #[error("{0}")]
    Internal(&'static str),
}
