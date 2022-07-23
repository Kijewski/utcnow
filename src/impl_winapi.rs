use core::convert::Infallible;
use core::fmt;

use winapi::shared::minwindef::FILETIME;
use winapi::um::sysinfoapi::GetSystemTimePreciseAsFileTime;

use crate::{Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = true;

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let mut ft = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    unsafe { GetSystemTimePreciseAsFileTime(&mut ft) };
    let ft = (ft.dwHighDateTime as u64 * 0x1_0000_0000) + ft.dwLowDateTime as u64;
    // https://stackoverflow.com/a/19709740/416224
    // epoch is Jan. 1, 1601: 134774 days to Jan. 1, 1970
    Ok(UtcTime {
        secs: ft.div_euclid(10_000_000) as i64 - 11644473600,
        nanos: ft.rem_euclid(10_000_000) as u32 * 100,
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
