use quickcheck::{Arbitrary, Gen};

use crate::UtcTime;

impl Arbitrary for UtcTime {
    fn arbitrary(gen: &mut Gen) -> Self {
        let (secs, nanos) = <(i64, u32)>::arbitrary(gen);
        let nanos = nanos % 1_000_000_000;
        unsafe { UtcTime::new_unchecked(secs, nanos) }
    }
}

#[cfg(all(test, not(miri)))]
#[quickcheck_macros::quickcheck]
fn minimal_test(value: UtcTime) {
    assert!(value.nanos.get() < 1_000_000_000);
}
