use core::fmt;

/// Keccak result type alias.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Keccak errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Generic failure.
    Fail,
    /// The provided output buffer is too small.
    OutputTooSmall,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::Fail => "generic failure",
            Error::OutputTooSmall => "output buffer too small",
        })
    }
}

impl Error {
    /// Converts a raw [`HashReturn`](ffi::HashReturn) value into a [`Result`].
    #[inline]
    pub fn from_raw(raw: ffi::HashReturn) -> Result<()> {
        match raw {
            ffi::HashReturn::KECCAK_SUCCESS => Ok(()),
            ffi::HashReturn::KECCAK_FAIL => Err(Error::Fail),
            ffi::HashReturn::KECCAK_BAD_HASHLEN => Err(Error::OutputTooSmall),
        }
    }

    /// Converts a raw integer return value into a [`Result`].
    #[inline]
    pub fn from_int(raw: i32) -> Result<()> {
        if raw == 0 {
            Ok(())
        } else {
            Err(Error::Fail)
        }
    }
}
