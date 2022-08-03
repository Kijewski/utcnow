use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

use crate::UtcTime;

impl<'de> Deserialize<'de> for UtcTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (secs, nanos) = <(i64, u32)>::deserialize(deserializer)?;
        Self::new(secs, nanos).ok_or_else(|| D::Error::custom("nanoseconds out of range"))
    }
}

impl Serialize for UtcTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Self { secs, nanos } = *self;
        (secs, nanos).serialize(serializer)
    }
}

#[cfg(test)]
#[test]
fn minimal_test() {
    // serialize
    let value = UtcTime {
        secs: 1_659_539_413,
        nanos: 885_457_394,
    };
    let string = serde_json::to_string(&value).unwrap();
    assert_eq!(string, "[1659539413,885457394]");

    // serialize < 1970
    let value = UtcTime {
        secs: -1_659_539_413,
        nanos: 885_457_394,
    };
    let string = serde_json::to_string(&value).unwrap();
    assert_eq!(string, "[-1659539413,885457394]");

    // deserialize
    let expected = UtcTime {
        secs: 1_659_539_413,
        nanos: 885_457_394,
    };
    let actual: UtcTime = serde_json::from_str("[1659539413,885457394]").unwrap();
    assert_eq!(actual, expected);

    // deserialize < 1970
    let expected = UtcTime {
        secs: -1_659_539_413,
        nanos: 885_457_394,
    };
    let actual: UtcTime = serde_json::from_str("[-1659539413,885457394]").unwrap();
    assert_eq!(actual, expected);

    // deserialize illegal nanos
    assert!(serde_json::from_str::<UtcTime>("[1659539413,8854573940]").is_err());
}
