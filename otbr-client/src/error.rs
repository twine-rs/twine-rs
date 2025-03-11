use thiserror::Error;
use zbus;

use twine_codec::error::TwineCodecError;

#[derive(Debug, Error)]
pub enum OtbrClientError {
    #[error("Codec error: {0}")]
    Codec(#[from] TwineCodecError),
    #[error("Dbus error: {0}")]
    Dbus(#[from] zbus::Error),
}
