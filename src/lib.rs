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
//! expressed as seconds + nanoseconds since [`1970-01-01`](https://en.wikipedia.org/w/index.php?title=Unix_time&oldid=1099912565 "Unix time")
//! in the [proleptic Gregorian calendar](https://en.wikipedia.org/w/index.php?title=Proleptic_Gregorian_calendar&oldid=1053300561).
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
//! * Emscripten
//! * FreeBSD
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
//! * Android
//! * Darwin
//! * Dragonfly
//! * Fuchsia
//! * Illumos
//! * iOS
//! * OpenBSD
//! * Redox
//! * Solaris
//!
//! Increasing the <abbr title="Minimum Supported Rust Version">msrv</abbr> for [tier-2](https://doc.rust-lang.org/nightly/rustc/platform-support.html) or
//! lower platforms will not be indicated as a breaking change to the semver version.
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

use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::time::Duration;
#[cfg(feature = "std")]
use std::time::SystemTime;

use crate::platform::OsError;

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
    nanos: u32,
}

impl UtcTime {
    /// Get the current time
    ///
    /// This method does the same as calling [`utcnow()`].
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

    /// Convert a [SystemTime]
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature = "std")] let _: () = {
    /// # use std::time::SystemTime;
    /// # use utcnow::UtcTime;
    /// let now = UtcTime::from_system_time(SystemTime::now()).unwrap();
    /// # };
    /// ```
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
    /// let timestamp = UtcTime::from_duration(Duration::from_secs(42)).unwrap();
    /// assert_eq!(timestamp.as_nanos(), 42_000_000_000);
    /// ```
    pub fn from_duration(value: Duration) -> Option<Self> {
        let secs = value.as_secs().try_into().ok()?;
        let nanos = value.subsec_nanos();
        Some(Self { secs, nanos })
    }

    /// Total number of whole seconds since epoch (1970-01-01 in UTC)
    #[inline]
    pub const fn as_secs(self) -> i64 {
        self.secs
    }

    /// Total number of whole milliseconds since epoch (1970-01-01 in UTC)
    pub const fn as_millis(self) -> i128 {
        (self.secs as i128 * 1_000) + (self.nanos as i128 / 1_000_000)
    }

    /// Total number of whole microseconds since epoch (1970-01-01 in UTC)
    pub const fn as_micros(self) -> i128 {
        (self.secs as i128 * 1_000_000) + (self.nanos as i128 / 1_000)
    }

    /// Total number of whole nanoseconds since epoch (1970-01-01 in UTC)
    pub const fn as_nanos(self) -> i128 {
        (self.secs as i128 * 1_000_000_000) + (self.nanos as i128)
    }

    /// Fractional number of milliseconds since epoch (1970-01-01 in UTC)
    pub const fn subsec_millis(self) -> u32 {
        self.nanos / 1_000_000
    }

    /// Fractional number of microseconds since epoch (1970-01-01 in UTC)
    pub const fn subsec_micros(self) -> u32 {
        self.nanos / 1_000
    }

    /// Fractional number of nanoseconds since epoch (1970-01-01 in UTC)
    #[inline]
    pub const fn subsec_nanos(self) -> u32 {
        self.nanos
    }

    /// Convert the timestamp to a [Duration] since epoch (1970-01-01 in UTC)
    #[inline]
    pub fn into_duration(self) -> core::result::Result<Duration, ConvertionError> {
        Ok(Duration::new(
            self.secs.try_into().map_err(|_| ConvertionError)?,
            self.nanos,
        ))
    }

    /// Convert the timestamp to a [SystemTime]
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn into_system_time(self) -> core::result::Result<SystemTime, ConvertionError> {
        SystemTime::UNIX_EPOCH
            .checked_add(self.try_into()?)
            .ok_or(ConvertionError)
    }
}

/// Get the current unix time, seconds since 1970-01-01 in UTC
///
/// Please see the [module level documentation](crate) for more information.
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
    type Error = ConvertionError;

    #[inline]
    fn try_from(value: UtcTime) -> Result<Self, ConvertionError> {
        value.into_duration()
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl TryFrom<UtcTime> for SystemTime {
    type Error = ConvertionError;

    #[inline]
    fn try_from(value: UtcTime) -> Result<Self, ConvertionError> {
        value.into_system_time()
    }
}

impl TryFrom<Duration> for UtcTime {
    type Error = ConvertionError;

    #[inline]
    fn try_from(value: Duration) -> Result<Self, ConvertionError> {
        Self::from_duration(value).ok_or(ConvertionError)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl TryFrom<SystemTime> for UtcTime {
    type Error = ConvertionError;

    #[inline]
    fn try_from(value: SystemTime) -> Result<Self, ConvertionError> {
        Self::from_system_time(value).ok_or(ConvertionError)
    }
}

/// A result type that default to [`Error`] as error type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Could not query system time
///
/// For many target platforms [`utcnow()`] cannot fail.
/// If this is true for the current target, then the constant `INFALLIBLE` will be `true`.
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
pub struct ConvertionError;

impl fmt::Display for ConvertionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("cannot convert a negative UtcTime")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for ConvertionError {}

#[cfg(test)]
#[test]
fn test_if_can_call() {
    let _ = utcnow().unwrap();
}
