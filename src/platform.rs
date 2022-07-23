use core::fmt;

use crate::{Error, Result, UtcTime};

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    Err(Error::OsError(OsError))
}

#[derive(Debug, Clone, Copy)]
pub struct OsError;

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "Platform is not implemented. Please file a bug report to https://github.com/Kijewski/utcnow/",
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OsError {}
