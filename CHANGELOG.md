# Changes between the versions

## 0.2.1 (2022-08-23)

* Implement `Display`, `FromStr`, and `TryFrom<&str>` for `UtcTime`
  ([#9](https://github.com/Kijewski/utcnow/pull/9))
* Add `UtcTime::new_unchecked()`
  ([#9](https://github.com/Kijewski/utcnow/pull/9))
* Add `rkyv` feature to implement `rkyv::Archive`, `rkyv::Serialize`, and `rkyv::Deserialize` for `UtcTime`
  ([#10](https://github.com/Kijewski/utcnow/pull/10))

## 0.2.0 (2022-08-04)

* Add method `UtcTime::new()`
  ([#6](https://github.com/Kijewski/utcnow/pull/6))
* Add optional features `serde`, `arbitrary`, `proptest`, `quickcheck`
  ([#6](https://github.com/Kijewski/utcnow/pull/6))
* Use niche optimization
  ([#7](https://github.com/Kijewski/utcnow/pull/7))

## 0.1.4 (2022-08-03)

* Make `from_duration()` and `into_duration()` const
  ([#3](https://github.com/Kijewski/utcnow/pull/3))
* Fix `clippy::pedantic` warnings
  ([#4](https://github.com/Kijewski/utcnow/pull/4))

## 0.1.3 (2022-07-25)

* Fix typo in type name
  ([#2](https://github.com/Kijewski/utcnow/pull/2))

## 0.1.2 (2022-07-25, *yanked*)

* Extend documentation

## 0.1.1 (2022-07-24, *yanked*)

* Fix typo in documentation

## 0.1.0 (2022-07-24, *yanked*)

* Initial release
