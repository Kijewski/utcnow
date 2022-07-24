use core::fmt;

use wasi::{clock_time_get, Errno, CLOCKID_REALTIME};

use crate::{Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = false;

pub(crate) fn utcnow() -> Result<UtcTime> {
    let nanos = unsafe { clock_time_get(CLOCKID_REALTIME, 100) }.map_err(OsError)?;
    Ok(UtcTime {
        secs: nanos.div_euclid(1_000_000_000) as i64,
        nanos: nanos.rem_euclid(1_000_000_000) as u32,
    })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(Errno);

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not query clock_gettime(): {}", self.0.message())
    }
}
