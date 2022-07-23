use core::fmt;
use core::ptr::slice_from_raw_parts;
use core::str::from_utf8_unchecked;

use libc::{clock_gettime, strerror, strlen, timespec, CLOCK_REALTIME};

use crate::{Error, Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = false;

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let result = unsafe { clock_gettime(CLOCK_REALTIME, &mut ts) };
    if result != 0 {
        return Err(Error(OsError(errno::errno().0)));
    }

    Ok(UtcTime {
        secs: ts.tv_sec,
        nanos: ts.tv_nsec as _,
    })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(i32);

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = unsafe { strerror(self.0) };
        let msg = match msg.is_null() {
            true => "?",
            false => unsafe {
                from_utf8_unchecked(&*slice_from_raw_parts(msg as *mut u8, strlen(msg)))
            },
        };
        write!(f, "error {}: {}", self.0, msg)
    }
}
