#![cfg_attr(docsrs, doc(cfg(feature = "castaway")))]

use castaway::LifetimeFree;

use crate::UtcTime;

unsafe impl LifetimeFree for UtcTime {}
