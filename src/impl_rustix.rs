use core::convert::Infallible;
use core::fmt;

use rustix::time::{clock_gettime, ClockId};

use crate::{Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = true;

#[allow(trivial_casts)]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let now = clock_gettime(ClockId::Realtime);
    let secs = now.tv_sec as i64; // tv_sec is i32 is emscripten
    let nanos = now.tv_nsec as u32;
    Ok(UtcTime { secs, nanos })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(Infallible);

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
