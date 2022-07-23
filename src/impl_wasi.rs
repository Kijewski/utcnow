use core::fmt;

use wasi::{clock_time_get, Errno, CLOCKID_REALTIME};

use crate::{Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = false;

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let ts = unsafe { clock_time_get(CLOCKID_REALTIME, 100) }.map_err(OsError)?;
    Ok(UtcTime {
        secs: ts.div_euclid(1_000_000_000) as i64,
        nanos: ts.rem_euclid(1_000_000_000) as u32,
    })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(Errno);

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error {}: {}", self.0.raw(), self.0.message())
    }
}
