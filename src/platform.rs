use core::fmt;

use crate::{Error, Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = false;
pub(crate) const INFALLIBLE: bool = false;

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    Err(Error::OsError(OsError))
}

#[derive(Debug, Clone, Copy)]
pub struct OsError;

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("platform is not implemented")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OsError {}

#[cfg(not(feature = "fallback"))]
compile_error!(
    "Target platform is not implemented. Please file a bug report to https://github.com/Kijewski/utcnow/",
);
