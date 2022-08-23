# utcnow — Get the current unixtime in a no-std context

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/utcnow/CI?logo=github)](https://github.com/Kijewski/utcnow/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/utcnow?logo=rust)](https://crates.io/crates/utcnow)
![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.48-important?logo=rust "Minimum Supported Rust Version")
[![License](https://img.shields.io/crates/l/utcnow?color=informational&logo=apache)](/LICENSE.md)

This library solves one question, and one question only: *What's the time?*

In [UTC](https://en.wikipedia.org/w/index.php?title=Coordinated_Universal_Time&oldid=1099753328 "Coordinated Universal Time"), and
according to the clock of the PC, tablet, toaster … the library runs on,
expressed as seconds + nanoseconds since [`1970-01-01`](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1099912565 "Unix time").

```rust
let now = utcnow().unwrap();
let seconds = now.as_secs();
let nanos = now.subsec_nanos();
```

For many target platforms this call cannot fail.
If this is true for the current target, then the constant `INFALLIBLE` will be `true`.

If the target platform is not supported, then `utcnow()` will always return an error instead of failing to compile.
Use the library with `default-features = false` and without the feature `fallback` to get a compile-time error instead.

The feature `std` (enabled by default) is only needed if you need the `Error` type to implement `std::error::Error`.

### Supported platforms

If you have successfully tested one of the untested targets, then please [tell me](https://github.com/Kijewski/utcnow/issues).
And if not, then even more so!

If you know how to implement another target, then please open a [pull request](https://github.com/Kijewski/utcnow/pulls).

**Supported and tested:**

* Android
* Emscripten
* FreeBSD
* Illumos
* Linux
* Linux with Musl
* MacOS
* NetBSD
* WASI
* wasm32
* Windows

**(Probably) supported, but not actually tested:**

* Darwin
* Dragonfly
* Fuchsia
* iOS
* OpenBSD
* Redox
* Solaris

Increasing the <abbr title="Minimum Supported Rust Version">msrv</abbr> for [tier-2](https://doc.rust-lang.org/nightly/rustc/platform-support.html) or
lower platforms will not be indicated as a breaking change to the semver version.

### Feature flags

`utcnow` has the following optional features:

* `serde`, which implements [`serde::Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html)
   and [`serde::Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html) for `UtcTime`.

* `arbitrary`, which implements the [`arbitrary::Arbitrary`](https://docs.rs/arbitrary/1/arbitrary/trait.Arbitrary.html) trait for `UtcTime`.

* `proptest`, which implements the [`proptest::arbitrary::Arbitrary`](https://docs.rs/proptest/1/proptest/arbitrary/trait.Arbitrary.html) trait for `UtcTime`.

* `quickcheck`, which implements the [`quickcheck::Arbitrary`](https://docs.rs/quickcheck/1/quickcheck/trait.Arbitrary.html) trait for `UtcTime`.

* `rkyv`, which implements the [`rkyv::Archive`](https://docs.rs/rkyv/0.7/rkyv/trait.Archive.html),
  [`rkyv::Serialize`](https://docs.rs/rkyv/0.7/rkyv/trait.Serialize.html),
  and [`rkyv::Deserialize`](https://docs.rs/rkyv/0.7/rkyv/trait.Deserialize.html) for `UtcTime`.

* `castaway`, which implements the [`castaway::LifetimeFree`](https://docs.rs/castaway/0.2/castaway/trait.LifetimeFree.html)
  trait for `UtcTime`.
