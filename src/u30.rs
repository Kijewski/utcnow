use core::{cmp, fmt, hash, mem};

/// A 30-bit unsigned integer
#[repr(transparent)]
#[derive(Copy, Clone)]
pub(crate) struct U30(Buf);

impl U30 {
    pub(crate) const ZERO: U30 = {
        #[cfg(target_endian = "little")]
        let a = 0;
        #[cfg(target_endian = "big")]
        let a = MaxByte::V0;

        #[cfg(target_endian = "little")]
        let d = MaxByte::V0;
        #[cfg(target_endian = "big")]
        let d = 0;

        let b = 0;
        let c = 0;
        let align = [];
        U30(Buf { align, a, b, c, d })
    };

    // Both methods compile to identity functions:

    /// SAFETY: the caller has to ensure that the value is in range
    #[allow(unconditional_panic)]
    #[allow(clippy::out_of_bounds_indexing)]
    #[inline]
    #[const_fn::const_fn("1.56")]
    pub(crate) const unsafe fn new_unchecked(value: u32) -> Self {
        if cfg!(debug_assertions) && value > 1_000_000_000 {
            let illegal_value = [];
            return illegal_value[0];
        }

        mem::transmute(value)
    }

    #[inline]
    #[const_fn::const_fn("1.56")]
    pub(crate) const fn get(self) -> u32 {
        unsafe { mem::transmute(self) }
    }
}

impl fmt::Debug for U30 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl fmt::Display for U30 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl cmp::PartialEq for U30 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl cmp::PartialOrd for U30 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Eq for U30 {}

impl cmp::Ord for U30 {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.get().cmp(&other.get())
    }
}

impl hash::Hash for U30 {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl Default for U30 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

#[repr(C)]
#[cfg(target_endian = "little")]
#[derive(Copy, Clone)]
struct Buf {
    align: [u32; 0],
    a: u8,
    b: u8,
    c: u8,
    d: MaxByte,
}

#[repr(C)]
#[cfg(target_endian = "big")]
#[derive(Copy, Clone)]
struct Buf {
    align: [u32; 0],
    a: MaxByte,
    b: u8,
    c: u8,
    d: u8,
}

#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum MaxByte {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    V10 = 10,
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
    V16 = 16,
    V17 = 17,
    V18 = 18,
    V19 = 19,
    V20 = 20,
    V21 = 21,
    V22 = 22,
    V23 = 23,
    V24 = 24,
    V25 = 25,
    V26 = 26,
    V27 = 27,
    V28 = 28,
    V29 = 29,
    V30 = 30,
    V31 = 31,
    V32 = 32,
    V33 = 33,
    V34 = 34,
    V35 = 35,
    V36 = 36,
    V37 = 37,
    V38 = 38,
    V39 = 39,
    V40 = 40,
    V41 = 41,
    V42 = 42,
    V43 = 43,
    V44 = 44,
    V45 = 45,
    V46 = 46,
    V47 = 47,
    V48 = 48,
    V49 = 49,
    V50 = 50,
    V51 = 51,
    V52 = 52,
    V53 = 53,
    V54 = 54,
    V55 = 55,
    V56 = 56,
    V57 = 57,
    V58 = 58,
    V59 = 59,
    V60 = 60,
    V61 = 61,
    V62 = 62,
    V63 = 63,
}
