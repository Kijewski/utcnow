use core::fmt;

use crate::{Error, Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = false;

#[allow(trivial_casts)]
#[allow(clippy::cast_lossless)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::unnecessary_cast)]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let mut now = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let result = unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut now) };
    if result != 0 {
        #[cfg(not(any(target_os = "emscripten", target_os = "haiku")))]
        return Err(Error(OsError(errno::errno().0)));

        #[cfg(any(target_os = "emscripten", target_os = "haiku"))]
        return Err(Error(OsError()));
    }

    let secs = now.tv_sec as i64; // tv_sec is i32 in emscripten
    let nanos = now.tv_nsec as u32;
    Ok(unsafe { UtcTime::new_unchecked(secs, nanos) })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(#[cfg(not(any(target_os = "emscripten", target_os = "haiku")))] i32);

#[allow(trivial_casts)] // msg is already `*mut u8` on thumbv7neon-linux-androideabi
impl fmt::Display for OsError {
    #[cfg(not(any(target_os = "emscripten", target_os = "haiku")))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match unsafe { libc::strerror(self.0) } {
            msg if msg.is_null() => {
                write!(f, "could not query clock_gettime(): errno {}", self.0)
            },
            msg => {
                use core::ptr::slice_from_raw_parts;
                use core::str::from_utf8_unchecked;

                let msg = unsafe {
                    from_utf8_unchecked(&*slice_from_raw_parts(msg.cast::<u8>(), libc::strlen(msg)))
                };
                write!(f, "could not query clock_gettime(): {}", msg)
            },
        }
    }

    #[cfg(any(target_os = "emscripten", target_os = "haiku"))]
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not query clock_gettime()")
    }
}
