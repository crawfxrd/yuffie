// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! The memory allocator to use with Rust.

use core::ptr::NonNull;

use crate::mem::MemoryType;
use crate::table::BootServices;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

// XXX: Put in `lib.rs`?
// TODO: Set to `None` on `ExitBootServices`
/// The Boot Services table holds the functions for memory allocation.
static mut BOOT_SERVICES: Option<NonNull<BootServices>> = None;

/// Save a reference to the Boot Services table.
pub(crate) fn init(bs: &mut BootServices) {
    unsafe {
        BOOT_SERVICES = NonNull::new(bs as *mut _);
    }
}

/// A convenience function to access the boot services table.
unsafe fn boot_services() -> NonNull<BootServices> {
    unsafe { BOOT_SERVICES.expect("boot services not available") }
}

pub struct Allocator;

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // TODO: Allow other memory types
        unsafe {
            boot_services()
                .as_ref()
                .allocate_pool(MemoryType::LOADER_DATA, layout.size())
                .expect("failed to allocate memory")
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe {
            boot_services().as_ref().free_pool(ptr).expect("failed to free memory");
        }
    }
}
