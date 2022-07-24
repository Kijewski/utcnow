use core::fmt;

use crate::{Error, Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = false;

#[allow(trivial_casts)]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let mut now = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let result = unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut now) };
    if result != 0 {
        #[cfg(not(target_os = "emscripten"))]
        return Err(Error(OsError(errno::errno().0)));

        #[cfg(target_os = "emscripten")]
        return Err(Error(OsError()));
    }

    let secs = now.tv_sec as i64; // tv_sec is i32 is emscripten
    let nanos = now.tv_nsec as u32;
    Ok(UtcTime { secs, nanos })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(#[cfg(not(target_os = "emscripten"))] i32);

#[allow(trivial_casts)] // msg is already `*mut u8` on thumbv7neon-linux-androideabi
impl fmt::Display for OsError {
    #[cfg(not(target_os = "emscripten"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match unsafe { libc::strerror(self.0) } {
            msg if msg.is_null() => {
                write!(f, "could not query clock_gettime(): errno {}", self.0)
            },
            msg => {
                use core::ptr::slice_from_raw_parts;
                use core::str::from_utf8_unchecked;

                let msg = unsafe {
                    from_utf8_unchecked(&*slice_from_raw_parts(msg as *mut u8, libc::strlen(msg)))
                };
                write!(f, "could not query clock_gettime(): {}", msg)
            },
        }
    }

    #[cfg(target_os = "emscripten")]
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not query clock_gettime()")
    }
}
