use super::{utcnow, Result, UtcTime, U30};

#[test]
#[allow(unknown_lints)]
#[allow(clippy::let_underscore_untyped)]
fn test_if_can_call() {
    let _ = utcnow().unwrap();
}

#[test]
fn test_layout() {
    use core::mem;

    assert_eq!(mem::align_of::<u32>(), mem::align_of::<U30>());
    assert_eq!(mem::size_of::<u32>(), mem::size_of::<U30>());
    assert_eq!(mem::size_of::<u32>(), mem::size_of::<Option<U30>>());

    assert_eq!(mem::size_of::<UtcTime>(), mem::size_of::<Option<UtcTime>>());
    assert_eq!(mem::size_of::<UtcTime>(), mem::size_of::<Result<UtcTime>>());
}

#[test]
fn test_parse() {
    assert_eq!(
        "1661201091.326474702123".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_702).unwrap(),
    );
    assert_eq!(
        "1661201091.32647470212".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_702).unwrap(),
    );
    assert_eq!(
        "1661201091.3264747021".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_702).unwrap(),
    );
    assert_eq!(
        "1661201091.326474702".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_702).unwrap(),
    );
    assert_eq!(
        "1661201091.32647470".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_700).unwrap(),
    );
    assert_eq!(
        "1661201091.3264747".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_700).unwrap(),
    );
    assert_eq!(
        "1661201091.326474".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_474_000).unwrap(),
    );
    assert_eq!(
        "1661201091.32647".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_470_000).unwrap(),
    );
    assert_eq!(
        "1661201091.3264".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_400_000).unwrap(),
    );
    assert_eq!(
        "1661201091.326".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 326_000_000).unwrap(),
    );
    assert_eq!(
        "1661201091.32".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 320_000_000).unwrap(),
    );
    assert_eq!(
        "1661201091.3".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 300_000_000).unwrap(),
    );
    assert_eq!(
        "1661201091.".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 000_000_000).unwrap(),
    );
    assert_eq!(
        "1661201091".parse::<UtcTime>().unwrap(),
        UtcTime::new(1_661_201_091, 000_000_000).unwrap(),
    );
    assert_eq!(
        "0.3264747021".parse::<UtcTime>().unwrap(),
        UtcTime::new(0, 326_474_702).unwrap(),
    );
    assert_eq!(
        ".3264747021".parse::<UtcTime>().unwrap(),
        UtcTime::new(0, 326_474_702).unwrap(),
    );

    assert!("".parse::<UtcTime>().is_err());
    assert!(".".parse::<UtcTime>().is_err());
    assert!(":D".parse::<UtcTime>().is_err());
    assert!("1661201091.3264747021²3".parse::<UtcTime>().is_err());
    assert!(" 1661201091".parse::<UtcTime>().is_err());
    assert!("1661201091 ".parse::<UtcTime>().is_err());
    assert!("1661201091. 1".parse::<UtcTime>().is_err());
    assert!("1661201091 .1".parse::<UtcTime>().is_err());
}
