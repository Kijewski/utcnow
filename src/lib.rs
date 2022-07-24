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
//! ![Minimum supported Rust version](https://img.shields.io/badge/rustc-%3F%3F%3F-important?logo=rust "Minimum Supported Rust Version")
//! [![License](https://img.shields.io/crates/l/utcnow?color=informational&logo=apache)](https://github.com/Kijewski/utcnow/blob/v0.0.0-pre1/LICENSE.md)
//!
//! Work in progress.
//!
//! ### Supported platforms
//!
//! * Darwin *(untested)*
//! * Dragonfly *(untested)*
//! * Emscripten
//! * FreeBSD
//! * iOS *(untested)*
//! * Linux
//! * MacOS
//! * NetBSD
//! * OpenBSD *(untested)*
//! * Redox *(untested)*
//! * WASI
//! * wasm32
//! * Windows
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
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(single_use_lifetimes)]
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
    any(target_os = "darwin", target_os = "netbsd", target_os = "emscripten"),
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

use crate::platform::OsError;

/// `true` if getting the time is implemented for the target platform
pub const IMPLEMENTED: bool = platform::IMPLEMENTED;

/// `true` if [`utcnow()`] cannot fail
pub const INFALLIBLE: bool = platform::INFALLIBLE;

/// A Unix time, i.e. seconds since 1970-01-01 in UTC
///
/// # Notice
///
/// Using [`i64`] values as seconds since 1970-01-01, this library will only work until
/// `Fri Apr 11 2262 23:47:16 GMT+0000`. If you need the library to work for later dates, please
/// open an issue no earlier than 2162-04-11.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UtcTime {
    /// Seconds since epoch
    secs: i64,
    /// Nanoseconds since epoch
    nanos: u32,
}

impl UtcTime {
    /// Total number of whole seconds since epoch (1970-01-01 in UTC)
    #[inline]
    pub fn as_secs(self) -> i64 {
        self.secs
    }

    /// Total number of whole milliseconds since epoch (1970-01-01 in UTC)
    pub fn as_millis(self) -> i128 {
        (self.secs as i128 * 1_000) + (self.nanos as i128 / 1_000_000)
    }

    /// Total number of whole microseconds since epoch (1970-01-01 in UTC)
    pub fn as_micros(self) -> i128 {
        (self.secs as i128 * 1_000_000) + (self.nanos as i128 / 1_000)
    }

    /// Total number of whole nanoseconds since epoch (1970-01-01 in UTC)
    pub fn as_nanos(self) -> i128 {
        (self.secs as i128 * 1_000_000_000) + (self.nanos as i128)
    }

    /// Fractional number of milliseconds since epoch (1970-01-01 in UTC)
    pub fn subsec_millis(self) -> u32 {
        self.nanos / 1_000_000
    }

    /// Fractional number of microseconds since epoch (1970-01-01 in UTC)
    pub fn subsec_micros(self) -> u32 {
        self.nanos / 1_000
    }

    /// Fractional number of nanoseconds since epoch (1970-01-01 in UTC)
    #[inline]
    pub fn subsec_nanos(self) -> u32 {
        self.nanos
    }
}

/// Get the current unix time, seconds since 1970-01-01 in UTC
#[inline]
pub fn utcnow() -> Result<UtcTime> {
    platform::utcnow()
}

impl TryFrom<UtcTime> for Duration {
    type Error = NegativeTime;

    fn try_from(value: UtcTime) -> Result<Self, NegativeTime> {
        Ok(Duration::new(
            value.secs.try_into().map_err(|_| NegativeTime)?,
            value.nanos,
        ))
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl TryFrom<UtcTime> for std::time::SystemTime {
    type Error = NegativeTime;

    fn try_from(value: UtcTime) -> Result<Self, NegativeTime> {
        std::time::SystemTime::UNIX_EPOCH
            .checked_add(value.try_into()?)
            .ok_or(NegativeTime)
    }
}

/// A result type that default to [`Error`] as error type
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Could not query system time
#[derive(Debug)]
#[allow(missing_copy_implementations)]
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

/// Cannot convert a negative UtcTime, i.e. before 1970-01-01
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NegativeTime;

impl fmt::Display for NegativeTime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("cannot convert a negative UtcTime")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for NegativeTime {}

#[cfg(test)]
#[test]
fn test_if_can_call() {
    let _ = utcnow().unwrap();
}
