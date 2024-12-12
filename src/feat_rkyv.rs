use core::{cmp, fmt, mem};

use rkyv::{Archive, Archived, Deserialize, Fallible, Resolver, Serialize};

use crate::UtcTime;
use crate::u30::U30;

/// An archived [`UtcTime`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchivedUtcTime {
    secs: Archived<i64>,
    nanos: Archived<u32>,
}

/// The resolver for an archived [`UtcTime`]
#[derive(Debug, Clone, Copy)]
pub struct UtcTimeResolver {
    secs: Resolver<i64>,
    nanos: Resolver<u32>,
}

impl Archive for UtcTime {
    type Archived = ArchivedUtcTime;
    type Resolver = UtcTimeResolver;

    #[allow(trivial_casts)]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (start, addr_secs) = {
            let offs_secs = &mut (*out).secs as *mut i64;
            (
                offs_secs.cast::<u8>().offset_from(out.cast::<u8>()) as usize,
                offs_secs,
            )
        };
        Archive::resolve(&self.secs, pos + start, resolver.secs, addr_secs);

        let (start, addr_nanos) = {
            let offs_nanos = &mut (*out).nanos as *mut u32;
            (
                offs_nanos.cast::<u8>().offset_from(out.cast::<u8>()) as usize,
                offs_nanos,
            )
        };
        let nanos: &u32 = mem::transmute(&self.nanos);
        Archive::resolve(nanos, pos + start, resolver.nanos, addr_nanos);
    }
}

impl<D: Fallible + ?Sized> Deserialize<UtcTime, D> for Archived<UtcTime> {
    fn deserialize(&self, deserializer: &mut D) -> Result<UtcTime, D::Error> {
        let secs = Deserialize::<i64, D>::deserialize(&self.secs, deserializer)?;

        let nanos: &u32 = unsafe { mem::transmute(&self.nanos) };
        let nanos = Deserialize::<u32, D>::deserialize(nanos, deserializer)?;
        let nanos = unsafe { U30::new_unchecked(nanos) };

        Ok(UtcTime { secs, nanos })
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for UtcTime {
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let secs = Serialize::<S>::serialize(&self.secs, serializer)?;

        let nanos: &u32 = unsafe { mem::transmute(&self.nanos) };
        let nanos = Serialize::<S>::serialize(nanos, serializer)?;

        Ok(UtcTimeResolver { secs, nanos })
    }
}

impl PartialEq<UtcTime> for ArchivedUtcTime {
    fn eq(&self, other: &UtcTime) -> bool {
        self.secs == other.secs && self.nanos == other.nanos.get()
    }
}

impl PartialEq<ArchivedUtcTime> for UtcTime {
    fn eq(&self, other: &ArchivedUtcTime) -> bool {
        self.secs == other.secs && self.nanos.get() == other.nanos
    }
}

impl PartialOrd<UtcTime> for ArchivedUtcTime {
    fn partial_cmp(&self, other: &UtcTime) -> Option<cmp::Ordering> {
        match self.secs.partial_cmp(&other.secs) {
            Some(cmp::Ordering::Equal) => self.nanos.partial_cmp(&other.nanos.get()),
            ord => ord,
        }
    }
}

impl PartialOrd<ArchivedUtcTime> for UtcTime {
    fn partial_cmp(&self, other: &ArchivedUtcTime) -> Option<cmp::Ordering> {
        match self.secs.partial_cmp(&other.secs) {
            Some(cmp::Ordering::Equal) => self.nanos.get().partial_cmp(&other.nanos),
            ord => ord,
        }
    }
}

impl fmt::Display for ArchivedUtcTime {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:09}", self.secs, self.nanos)
    }
}

#[cfg(test)]
#[test]
fn test() {
    let value = UtcTime::new(1_661_209_811, 467_621_425).unwrap();

    let bytes = rkyv::to_bytes::<_, 32>(&value).unwrap();
    let archived = unsafe { rkyv::archived_root::<UtcTime>(&bytes) };
    let deserialized: UtcTime = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(&value, archived);
    assert_eq!(&value, &deserialized);
    assert_eq!(archived, &deserialized);
}
