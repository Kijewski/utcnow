use core::convert::Infallible;
use core::fmt;

use winapi::shared::minwindef::FILETIME;
use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;

use crate::{Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = true;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let mut now = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    unsafe { GetSystemTimePreciseAsFileTime(&mut now) };
    let now = (now.dwHighDateTime as u64 * 0x1_0000_0000) + now.dwLowDateTime as u64;

    // https://stackoverflow.com/a/19709740/416224
    // epoch is Jan. 1, 1601: 134774 days to Jan. 1, 1970
    let secs = now.div_euclid(10_000_000) as i64 - 11644473600;
    let nanos = now.rem_euclid(10_000_000) as u32 * 100;
    Ok(unsafe { UtcTime::new_unchecked(secs, nanos) })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(Infallible);

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
