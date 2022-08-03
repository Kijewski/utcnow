use proptest::arbitrary::{Arbitrary, StrategyFor};
use proptest::prelude::any_with;
use proptest::strategy::{Map, Strategy};

use crate::UtcTime;

impl Arbitrary for UtcTime {
    type Parameters = <(i64, u32) as Arbitrary>::Parameters;
    type Strategy = Map<StrategyFor<(i64, u32)>, fn((i64, u32)) -> Self>;

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with::<(i64, u32)>(args).prop_map(|(secs, nanos)| {
            let nanos = nanos % 1_000_000_000;
            unsafe { UtcTime::create(secs, nanos) }
        })
    }
}

proptest::proptest! {
    #[cfg(all(test, not(miri)))]
    #[test]
    fn minimal_test(value: crate::UtcTime) {
        assert!(value.nanos.get() < 1_000_000_000);
    }
}
