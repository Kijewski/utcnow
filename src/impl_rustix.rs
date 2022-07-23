use core::convert::Infallible;
use core::fmt;

use rustix::time::{clock_gettime, ClockId};

use crate::{Result, UtcTime};

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let now = clock_gettime(ClockId::Realtime);
    Ok(UtcTime {
        secs: now.tv_sec,
        nanos: now.tv_nsec as u32,
    })
}

#[derive(Debug, Clone, Copy)]
pub enum OsError {
    Infallible(Infallible),
}

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsError::Infallible(_) => Ok(()),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OsError {}
