// Copyright (C) Microsoft Corporation. All rights reserved.

//! Module for device dma support.

// UNSAFETY: This is required to implement the MappedDmaTarget trait which
// unsafe because of it's requirement for the implementer to keep the
// `base()..len()` mapped for the lifetime of the struct.
#![allow(unsafe_code)]

use crate::SharedPoolHandle;
use user_driver::memory::MappedDmaTarget;

/// Shared memory representing a DMA buffer useable by devices.
pub struct SharedDmaBuffer {
    pub(crate) mapping: sparse_mmap::SparseMapping,
    // Holds allocation until dropped.
    pub(crate) _alloc: SharedPoolHandle,
    pub(crate) pfns: Vec<u64>,
}

/// SAFETY: This struct keeps both the shared memory region which the sparse
/// mapping maps, along with the sparse mapping itself until the struct is drop,
/// satisfying the trait.
unsafe impl MappedDmaTarget for SharedDmaBuffer {
    fn base(&self) -> *const u8 {
        self.mapping.as_ptr() as *const u8
    }

    fn len(&self) -> usize {
        self.mapping.len()
    }

    fn pfns(&self) -> &[u64] {
        &self.pfns
    }
}