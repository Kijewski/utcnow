// SPDX-License-Identifier: Apache-2.0
//
// Copyright 2022 René Kijewski <crates.io@k6i.de>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#![allow(unknown_lints)]
#![allow(clippy::doc_markdown)]

//! # utcnow — Get the current unixtime in a no-std context
//!
//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/utcnow/CI?logo=github)](https://github.com/Kijewski/tzdb/actions/workflows/ci.yml)
//! [![Crates.io](https://img.shields.io/crates/v/utcnow?logo=rust)](https://crates.io/crates/utcnow)
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-1.48-important?logo=rust "Minimum Supported Rust Version")
//! [![License](https://img.shields.io/crates/l/utcnow?color=informational&logo=apache)](https://github.com/Kijewski/utcnow/blob/v0.0.0-pre1/LICENSE.md)
//!
//! This library solves one question, and one question only: *What's the time?*
//!
//! In [UTC](https://en.wikipedia.org/w/index.php?title=Coordinated_Universal_Time&oldid=1099753328 "Coordinated Universal Time"), and
//! according to the clock of the PC, tablet, toaster … the library runs on,
//! expressed as seconds + nanoseconds since [`1970-01-01`](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1099912565 "Unix time").
//!
//! ```rust
//! # use utcnow::utcnow;
//! let now = utcnow().unwrap();
//! let seconds = now.as_secs();
//! let nanos = now.subsec_nanos();
//! ```
//!
//! For many target platforms this call cannot fail.
//! If this is true for the current target, then the constant [`INFALLIBLE`] will be `true`.
//!
//! If the target platform is not supported, then [`utcnow()`] will always return an error instead of failing to compile.
//! Use the library with `default-features = false` and without the feature `"fallback"` to get a compile-time error instead.
//!
//! The feature `"std"` (enabled by default) is only needed if you need the [`Error`] type to implement [`std::error::Error`].
//!
//! ### Supported platforms
//!
//! If you have successfully tested one of the untested targets, then please [tell me](https://github.com/Kijewski/utcnow/issues).
//! And if not, then even more so!
//!
//! If you know how to implement another target, then please open a [pull request](https://github.com/Kijewski/utcnow/pulls).
//!
//! **Supported and tested:**
//!
//! * Android
//! * Emscripten
//! * FreeBSD
//! * Haiku
//! * Illumos
//! * Linux
//! * Linux with Musl
//! * MacOS
//! * NetBSD
//! * WASI
//! * wasm32
//! * Windows
//!
//! **(Probably) supported, but not actually tested:**
//!
//! * Darwin
//! * Dragonfly
//! * Fuchsia
//! * iOS
//! * OpenBSD
//! * Redox
//! * Solaris
//!
//! Increasing the <abbr title="Minimum Supported Rust Version">msrv</abbr> for [tier-2](https://doc.rust-lang.org/nightly/rustc/platform-support.html) or
//! lower platforms will not be indicated as a breaking change to the semver version.
//!
//! ### Feature flags
//!
//! `utcnow` has the following optional features:
//!
//! * `serde`, which implements [`serde::Deserialize`] and [`serde::Serialize`] for [`UtcTime`].
//!
//! * `arbitrary`, which implements the [`arbitrary::Arbitrary`] trait for [`UtcTime`].
//!
//! * `proptest`, which implements the [`proptest::arbitrary::Arbitrary`] trait for [`UtcTime`].
//!
//! * `quickcheck`, which implements the [`quickcheck::Arbitrary`] trait for [`UtcTime`].
//!
//! * `rkyv`, which implements the [`rkyv::Archive`], [`rkyv::Serialize`], and [`rkyv::Deserialize`] for [`UtcTime`].
//!
//! * `castaway`, which implements the [`castaway::LifetimeFree`] trait for [`UtcTime`].
//!

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unused_attributes)]
#![warn(absolute_paths_not_starting_with_crate)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(meta_variable_misuse)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unused_extern_crates)]
#![warn(unused_lifetimes)]
#![warn(unused_results)]

#[cfg(docsrs)]
pub mod changelog;
#[cfg(feature = "arbitrary")]
mod feat_arbitrary;
#[cfg(feature = "castaway")]
mod feat_castaway;
#[cfg(feature = "proptest")]
mod feat_proptest;
#[cfg(feature = "quickcheck")]
mod feat_quickcheck;
#[cfg(feature = "rkyv")]
mod feat_rkyv;
#[cfg(feature = "serde")]
mod feat_serde;
#[cfg_attr(
    any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "linux",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "redox",
    ),
    path = "impl_rustix.rs"
)]
#[cfg_attr(
    any(
        target_os = "android",
        target_os = "darwin",
        target_os = "emscripten",
        target_os = "fuchsia",
        target_os = "haiku",
        target_os = "illumos",
        target_os = "netbsd",
        target_os = "solaris",
    ),
    path = "impl_libc.rs"
)]
#[cfg_attr(target_os = "wasi", path = "impl_wasi.rs")]
#[cfg_attr(target_os = "windows", path = "impl_winapi.rs")]
#[cfg_attr(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    path = "impl_web.rs"
)]
mod platform;
#[cfg(test)]
mod test;
mod u30;

use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::time::Duration;
#[cfg(feature = "std")]
use std::time::SystemTime;

use crate::platform::OsError;
use crate::u30::U30;

/// `true` if getting the time is implemented for the target platform
pub const IMPLEMENTED: bool = platform::IMPLEMENTED;

/// `true` if [`utcnow()`] cannot fail
pub const INFALLIBLE: bool = platform::INFALLIBLE;

/// A Unix time, i.e. seconds since 1970-01-01 in UTC
///
/// Using [`i64`] values as seconds since 1970-01-01, this library will work for the next 292 billion years.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UtcTime {
    /// Seconds since epoch
    secs: i64,
    /// Nanoseconds since epoch
    nanos: U30,
}

impl UtcTime {
    /// Start of the [Unix time](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1099912565) epoch, 1970-01-01.
    pub const EPOCH: UtcTime = UtcTime {
        secs: 0,
        nanos: U30::ZERO,
    };

    /// Get the current time
    ///
    /// This method does the same as calling [`utcnow()`].
    ///
    /// # Errors
    ///
    /// See [`utcnow()`] for further information.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let seconds = now.as_secs();
    /// let nanos = now.subsec_nanos();
    /// ```
    #[inline]
    pub fn now() -> Result<Self> {
        utcnow()
    }

    /// Build a new [`UtcTime`] without normalization
    ///
    /// If the value for `nanos` exceeds `1_000_000_000`, your program is malformed.
    /// Expect undefined behavior!
    #[inline]
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub unsafe fn new_unchecked(secs: i64, nanos: u32) -> Self {
        let nanos = U30::new_unchecked(nanos);
        Self { secs, nanos }
    }

    /// Build a new [`UtcTime`]
    ///
    /// `nanos` will be normalized to a values less than `1_000_000_000`, the number of nanoseconds in a second.
    /// If the resulting number of seconds will exceed [`i64::MAX`], [`None`] is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use utcnow::UtcTime;
    /// // August 3, 2022, about 19 o'clock in the evening in CEST.
    /// let timestamp = UtcTime::new(1_659_545_693, 895_531_827).unwrap();
    /// ```
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub fn new(secs: i64, nanos: u32) -> Option<Self> {
        const NANOS_PER_SEC: u32 = 1_000_000_000;

        if nanos < NANOS_PER_SEC {
            return Some(unsafe { Self::new_unchecked(secs, nanos) });
        }

        let extra_seconds = nanos / NANOS_PER_SEC;
        let nanos = nanos % NANOS_PER_SEC;
        match secs.checked_add(extra_seconds as i64) {
            Some(secs) => Some(unsafe { Self::new_unchecked(secs, nanos) }),
            None => None,
        }
    }

    /// Convert a [SystemTime]
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] let _: () = {
    /// # use std::time::SystemTime;
    /// # use utcnow::UtcTime;
    /// let system_time = SystemTime::now();
    /// let now = UtcTime::from_system_time(system_time).unwrap();
    /// # };
    /// ```
    #[must_use]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn from_system_time(value: SystemTime) -> Option<Self> {
        Self::from_duration(value.duration_since(SystemTime::UNIX_EPOCH).ok()?)
    }

    /// Convert a [Duration]
    ///
    /// The duration is interpreted as seconds since epoch (1970-01-01 in UTC).
    ///
    /// # Example
    ///
    /// ```
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let duration = Duration::from_secs(42);
    /// let timestamp = UtcTime::from_duration(duration).unwrap();
    /// assert_eq!(timestamp.as_nanos(), 42_000_000_000);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[const_fn::const_fn("1.56")]
    pub fn from_duration(value: Duration) -> Option<Self> {
        const I64_MAX: u64 = i64::MAX as u64;
        let secs = match value.as_secs() {
            secs @ 0..=I64_MAX => secs as i64,
            _ => return None,
        };
        let nanos = value.subsec_nanos();
        Some(unsafe { Self::new_unchecked(secs, nanos) })
    }

    /// Total number of whole seconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let total_secs = now.as_secs();
    /// assert!(total_secs > 1_658_711_810);
    /// assert!(total_secs < 1_974_324_043); // update before 2032-07-25
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_secs(self) -> i64 {
        self.secs
    }

    /// Total number of whole milliseconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let total_millis = now.as_millis();
    /// assert!(total_millis > 1_658_711_810_802);
    /// assert!(total_millis < 1_974_324_043_000); // update before 2032-07-25
    /// ```
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub fn as_millis(self) -> i128 {
        (self.secs as i128 * 1_000) + (self.nanos.get() as i128 / 1_000_000)
    }

    /// Total number of whole microseconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let total_micros = now.as_micros();
    /// assert!(total_micros > 1_658_711_810_802_520);
    /// assert!(total_micros < 1_974_324_043_000_000); // update before 2032-07-25
    /// ```
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub fn as_micros(self) -> i128 {
        (self.secs as i128 * 1_000_000) + (self.nanos.get() as i128 / 1_000)
    }

    /// Total number of whole nanoseconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let total_nanos = now.as_nanos();
    /// assert!(total_nanos > 1_658_711_810_802_520_027);
    /// assert!(total_nanos < 1_974_324_043_000_000_000); // update before 2032-07-25
    /// ```
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub fn as_nanos(self) -> i128 {
        (self.secs as i128 * 1_000_000_000) + (self.nanos.get() as i128)
    }

    /// Fractional number of milliseconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let millis = now.subsec_millis();
    /// assert!(millis < 1_000);
    /// ```
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub fn subsec_millis(self) -> u32 {
        self.nanos.get() / 1_000_000
    }

    /// Fractional number of microseconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let micros = now.subsec_micros();
    /// assert!(micros < 1_000_000);
    /// ```
    #[must_use]
    #[const_fn::const_fn("1.56")]
    pub fn subsec_micros(self) -> u32 {
        self.nanos.get() / 1_000
    }

    /// Fractional number of nanoseconds since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let nanos = now.subsec_nanos();
    /// assert!(nanos < 1_000_000_000);
    /// ```
    #[must_use]
    #[inline]
    #[const_fn::const_fn("1.56")]
    pub fn subsec_nanos(self) -> u32 {
        self.nanos.get()
    }

    /// Convert the timestamp to a [Duration] since epoch (1970-01-01 in UTC)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use core::time::Duration;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let duration = now.into_duration().unwrap();
    /// ```
    #[allow(clippy::cast_sign_loss)]
    #[const_fn::const_fn("1.58")]
    pub fn into_duration(self) -> Result<Duration, ConversionError> {
        let secs = match self.secs {
            secs @ 0..=i64::MAX => secs as u64,
            _ => return Err(ConversionError),
        };
        Ok(Duration::new(secs, self.nanos.get()))
    }

    /// Convert the timestamp to a [SystemTime]
    ///
    /// # Errors
    ///
    /// The conversion won't succeed if and only if the stored date is earlier than 1970-01-01.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[cfg(feature = "std")] let _: () = {
    /// # use std::time::SystemTime;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::now().unwrap();
    /// let system_time = now.into_system_time().unwrap();
    /// # };
    /// ```
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn into_system_time(self) -> Result<SystemTime, ConversionError> {
        SystemTime::UNIX_EPOCH
            .checked_add(self.into_duration()?)
            .ok_or(ConversionError)
    }
}

impl fmt::Display for UtcTime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:09}", self.secs, self.nanos)
    }
}

impl TryFrom<&str> for UtcTime {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if matches!(value, "" | ".") || !value.is_ascii() {
            return Err(ConversionError);
        }

        // Only present since 1.52:
        // let (secs, nanos) = value.split_once('.').unwrap_or((value, ""));

        let (secs, nanos) = match value
            .as_bytes()
            .iter()
            .enumerate()
            .find(|(_, &c)| c == b'.')
        {
            Some((idx, _)) => unsafe {
                // SAFETY: we checked that `value` is ASCII, and we know that the index is valid
                (value.get_unchecked(..idx), value.get_unchecked(idx + 1..))
            },
            None => (value, ""),
        };

        let secs = match secs {
            "" => 0,
            secs => secs.parse().map_err(|_| ConversionError)?,
        };
        let nanos = match nanos {
            "" => 0,
            nanos => {
                let (nanos, factor) = if nanos.len() <= 9 {
                    let factor = match nanos.len() {
                        8 => 10,
                        7 => 100,
                        6 => 1000,
                        5 => 10000,
                        4 => 100_000,
                        3 => 1_000_000,
                        2 => 10_000_000,
                        1 => 100_000_000,
                        _ => 1,
                    };
                    (nanos, factor)
                } else {
                    // SAFETY: We checked that `value` is ASCII, so every substring is ASCII,
                    //         and we just checked that `nanos` is longer that 9 bytes.
                    let nanos = unsafe { nanos.get_unchecked(..9) };
                    let suffix = unsafe { nanos.get_unchecked(9..) };
                    if suffix.as_bytes().iter().any(|c| !matches!(c, b'0'..=b'9')) {
                        return Err(ConversionError);
                    }
                    (nanos, 1)
                };
                nanos.parse::<u32>().map_err(|_| ConversionError)? * factor
            },
        };
        Ok(unsafe { Self::new_unchecked(secs, nanos) })
    }
}

impl core::str::FromStr for UtcTime {
    type Err = ConversionError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

/// Get the current unix time, seconds since 1970-01-01 in UTC
///
/// Please see the [module level documentation](crate) for more information.
///
/// # Errors
///
/// For many target platforms this call cannot fail.
/// If this is true for the current target, then the constant [`INFALLIBLE`] will be `true`.
/// Rust will automatically optimize the [`unwrap()`](Result::unwrap) call into a no-op in this case.
/// Independent of the target platform the [`Error`] type will always be [`Send`] + [`Sync`] + [`Copy`].
///
/// # Example
///
/// ```rust
/// let now = utcnow::utcnow().unwrap();
/// let seconds = now.as_secs();
/// let nanos = now.subsec_nanos();
/// ```
#[inline]
pub fn utcnow() -> Result<UtcTime> {
    platform::utcnow()
}

impl TryFrom<UtcTime> for Duration {
    type Error = ConversionError;

    #[inline]
    fn try_from(value: UtcTime) -> Result<Self, ConversionError> {
        value.into_duration()
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl TryFrom<UtcTime> for SystemTime {
    type Error = ConversionError;

    #[inline]
    fn try_from(value: UtcTime) -> Result<Self, ConversionError> {
        value.into_system_time()
    }
}

impl TryFrom<Duration> for UtcTime {
    type Error = ConversionError;

    #[inline]
    fn try_from(value: Duration) -> Result<Self, ConversionError> {
        Self::from_duration(value).ok_or(ConversionError)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl TryFrom<SystemTime> for UtcTime {
    type Error = ConversionError;

    #[inline]
    fn try_from(value: SystemTime) -> Result<Self, ConversionError> {
        Self::from_system_time(value).ok_or(ConversionError)
    }
}

/// A result type that default to [`Error`] as error type
///
/// For many target platforms [`utcnow()`] cannot fail.
/// If this is true for the current target, then the constant [`INFALLIBLE`] will be `true`.
/// Rust will automatically optimize the [`unwrap()`](Result::unwrap) call into a no-op in this case.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Could not query system time
///
/// For many target platforms [`utcnow()`] cannot fail.
/// If this is true for the current target, then the constant [`INFALLIBLE`] will be `true`.
/// Independent of the target platform the [`Error`] type will always be [`Send`] + [`Sync`] + [`Copy`].
#[derive(Debug, Clone, Copy)]
pub struct Error(OsError);

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<OsError> for Error {
    #[inline]
    fn from(err: OsError) -> Self {
        Self(err)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

/// Could not convert from or to a [`UtcTime`]
///
/// You cannot convert a negative [`UtcTime`]  (i.e. before 1970-01-01) into a [`SystemTime`] or [`Duration`].
/// You cannot convert a [`SystemTime`] or [`Duration`] later than year 292 billion into a [`UtcTime`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConversionError;

impl fmt::Display for ConversionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("cannot convert a negative UtcTime")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for ConversionError {}

const _: () = {
    trait NoSuchTrait {}

    impl<T: ?Sized> NoSuchTrait for T {}

    #[cfg(has_core__panic__RefUnwindSafe)]
    use core::panic::RefUnwindSafe;
    #[cfg(has_core__panic__UnwindSafe)]
    use core::panic::UnwindSafe;

    #[cfg(not(has_core__panic__RefUnwindSafe))]
    use NoSuchTrait as RefUnwindSafe;
    #[cfg(not(has_core__panic__UnwindSafe))]
    use NoSuchTrait as UnwindSafe;

    trait AutoTraits {
        const AUTO_TRAITS: bool;
    }

    impl<T> AutoTraits for T
    where
        T: 'static + RefUnwindSafe + Send + Sync + Unpin + UnwindSafe,
    {
        const AUTO_TRAITS: bool = true;
    }

    const _: bool = ConversionError::AUTO_TRAITS;
    const _: bool = Error::AUTO_TRAITS;
    const _: bool = Option::<U30>::AUTO_TRAITS;
    const _: bool = OsError::AUTO_TRAITS;
    const _: bool = Result::<U30>::AUTO_TRAITS;
    const _: bool = U30::AUTO_TRAITS;
    const _: bool = UtcTime::AUTO_TRAITS;
};
