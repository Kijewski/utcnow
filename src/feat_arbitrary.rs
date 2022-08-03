use arbitrary::{Arbitrary, Result, Unstructured};

use crate::UtcTime;

impl<'a> Arbitrary<'a> for UtcTime {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let (secs, nanos) = <(i64, u32)>::arbitrary(u)?;
        let nanos = nanos % 1_000_000_000;
        Ok(Self { secs, nanos })
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <(i64, u32)>::size_hint(depth)
    }
}

#[cfg(all(test, not(miri)))]
#[test]
fn minimal_test() {
    let mut data = Unstructured::new(&[123; 16]);
    let value = UtcTime::arbitrary(&mut data).unwrap();
    assert!(value.nanos < 1_000_000_000);
}
