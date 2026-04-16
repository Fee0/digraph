use core::fmt;

/// Errors returned by PNG rendering (`feature = "image"`).
#[cfg(feature = "image")]
#[derive(Debug)]
pub enum RenderError {
    /// Underlying I/O error when writing an image.
    Io(std::io::Error),
    /// PNG encoding failed.
    Encode(image::ImageError),
}

#[cfg(feature = "image")]
impl From<std::io::Error> for RenderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[cfg(feature = "image")]
impl From<image::ImageError> for RenderError {
    fn from(value: image::ImageError) -> Self {
        Self::Encode(value)
    }
}

#[cfg(feature = "image")]
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::Io(e) => write!(f, "I/O error: {e}"),
            RenderError::Encode(e) => write!(f, "image encode error: {e}"),
        }
    }
}

#[cfg(feature = "image")]
impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RenderError::Io(e) => Some(e),
            RenderError::Encode(e) => Some(e),
        }
    }
}
