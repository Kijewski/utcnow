use core::convert::Infallible;
use core::fmt;

use crate::{Result, UtcTime};

#[inline]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let ms = js_sys::Date::now();
    Ok(UtcTime {
        secs: ms.div_euclid(1000_f64) as i64,
        nanos: ms.rem_euclid(1000_f64) as u32 * 1_000_000,
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        let now = crate::utcnow().unwrap();
        console_log!("now={:?}", now);
    }
}
