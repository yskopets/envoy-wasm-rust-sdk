use super::WasmResult;
use core::fmt;
use core::num::NonZeroU32;

/// A raw error returned by Envoy ABI, internally containing a 32-bit error
/// code.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Error {
    code: NonZeroU32,
}

impl Error {
    /// Constructs a new error from a raw error code, returning `None` if the
    /// error code is zero (which means success).
    pub fn from_raw_error(error: WasmResult) -> Option<Error> {
        Some(Error {
            code: NonZeroU32::new(error)?,
        })
    }

    /// Returns the raw error code that this error represents.
    pub fn raw_error(&self) -> u32 {
        self.code.get()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(error {})",
            self.code
        )?;
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("code", &self.code)
            .finish()
    }
}

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
impl std::error::Error for Error {}
