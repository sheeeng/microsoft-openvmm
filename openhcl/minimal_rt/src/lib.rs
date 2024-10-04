// Copyright (C) Microsoft Corporation. All rights reserved.

//! Minimal support runtime for `openhcl_boot` and other kernel-mode
//! environments.

#![no_std]
// UNSAFETY: Interacting with low level hardware and bootloader primitives.
#![allow(unsafe_code)]
#![warn(missing_docs)]

pub mod arch;
pub mod enlightened_panic;
pub mod reftime;
pub mod reloc;
pub mod rt;