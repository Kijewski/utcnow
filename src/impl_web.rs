use core::convert::Infallible;
use core::fmt;

use crate::{Result, UtcTime};

pub(crate) const IMPLEMENTED: bool = true;
pub(crate) const INFALLIBLE: bool = true;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn utcnow() -> Result<UtcTime> {
    let millis = js_sys::Date::now();
    Ok(UtcTime {
        secs: millis.div_euclid(1000_f64) as i64,
        nanos: millis.rem_euclid(1000_f64) as u32 * 1_000_000,
    })
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OsError(Infallible);

impl fmt::Display for OsError {
    #[inline]
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn pass() {
        let now = crate::utcnow().unwrap();
        wasm_bindgen_test::console_log!("now={:?}", now);
    }
}
