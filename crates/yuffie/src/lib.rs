// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Yuffie
//!
//! A library for creating UEFI applications, libraries, and drivers.
//!
//! ## References
//!
//! - [UEFI Specification, version 2.10][UEFI Spec]
//! - [UEFI Platform Initialization Specification, version 1.8][UEFI PI]
//!
//! [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_10_Aug29.pdf
//! [UEFI PI]: https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf

#![no_std]
#![allow(non_snake_case)]
#![deny(unused_crate_dependencies)]
#![deny(unused_imports)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::inline_asm_x86_att_syntax)]
#![deny(clippy::panic)]
#![deny(clippy::ref_binding_to_reference)]
//#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::wildcard_dependencies)]
#![warn(clippy::borrow_as_ptr)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::default_union_representation)]
#![warn(clippy::float_arithmetic)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::match_same_arms)]
//#![warn(clippy::missing_errors_doc)]
#![warn(clippy::ptr_as_ptr)]
#![warn(clippy::same_name_method)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::shadow_unrelated)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
pub mod global_alloc;

// Core
pub mod guid;
pub mod hii;
pub mod mem;
pub mod prelude;
pub mod status;
pub mod table;

// Protocols
pub mod proto;

use core::ptr;

use prelude::*;

// Ref: 4.1 UEFI Image Entry Point
/// Function signature for the entry point of a UEFI application or driver.
pub type ImageEntryFn = extern "efiapi" fn(Handle, &mut SystemTable) -> Status;

/// Cached pointer to the system table (edk2: `gST`).
static mut SYSTEM_TABLE: Option<ptr::NonNull<SystemTable>> = None;

/// A convenience function to access the system table.
pub fn system_table() -> &'static mut SystemTable {
    unsafe { SYSTEM_TABLE.expect("system table not available").as_mut() }
}

/// Initialize the library.
#[cfg(not(feature = "alloc"))]
pub fn init(st: &mut SystemTable) {
    unsafe {
        SYSTEM_TABLE = ptr::NonNull::new(st as *mut _);
    }
}

/// Initialize the library.
#[cfg(feature = "alloc")]
pub fn init(st: &mut SystemTable) {
    unsafe {
        SYSTEM_TABLE = ptr::NonNull::new(st as *mut _);
        global_alloc::init(st.boot_services());
    }
}

/// A collection of related interfaces.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Handle(ptr::NonNull<usize>);

impl Handle {
    /// Create an uninitialized handle for calling into a UEFI interface that
    /// will initialize it.
    ///
    /// ## Safety
    ///
    /// This must only be used as an out parameter to acquire a valid handle.
    /// It is invalid to use this value to an interface that expects a handle
    /// as an input parameter.
    pub unsafe fn uninit() -> Self {
        Handle(ptr::NonNull::dangling())
    }
}

/// Task Priority Level
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Tpl(usize);

impl Tpl {
    pub const NONE: Self = Self(0);
    pub const APPLICATION: Self = Self(4);
    pub const CALLBACK: Self = Self(8);
    pub const NOTIFY: Self = Self(16);
    pub const HIGH_LEVEL: Self = Self(31);
}

/// Handle to an event structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Event(ptr::NonNull<usize>);

impl Event {
    /// Create an uninitialized event handle for calling into a UEFI interface
    /// that will initialize it.
    ///
    /// ## Safety
    ///
    /// This must only be used as an out parameter to acquire a valid handle.
    /// It is invalid to use this value to an interface that expects a handle
    /// as an input parameter.
    pub unsafe fn uninit() -> Self {
        Self(ptr::NonNull::dangling())
    }
}

/// Event type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct EventType(u32);

impl EventType {
    pub const NONE: Self = Self(0);
    /// The event is a timer and may be passed to `SetTimer()`.
    pub const TIMER: Self = Self(0x8000_0000);
    /// The event is allocated from run-time memory.
    pub const RUNTIME: Self = Self(0x4000_0000);
    /// If an event of this type is not already in the signaled state, then the
    /// event's notification function will be queued at the event's TPL
    /// whenever the event is being waited on.
    pub const NOTIFY_WAIT: Self = Self(0x0000_0100);
    /// The event's notification function is queued whenever the event is
    /// signaled.
    pub const NOTIFY_SIGNAL: Self = Self(0x0000_0200);
    /// The event is notified when `ExitBootServices()` is performed.
    pub const SIGNAL_EXIT_BOOT_SERVICES: Self = Self(0x0000_0201);
    /// The event is notified when `SetVirtualAddressMap()` is performed.
    pub const SIGNAL_VIRTUAL_ADDRESS_CHANGE: Self = Self(0x6000_0202);
}

#[cfg(feature = "panic_handler")]
#[doc(hidden)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: Print panic info
    loop {
        unsafe {
            // TODO: Handle other architectures
            core::arch::asm!("hlt");
        }
    }
}
