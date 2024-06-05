// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Driver binding protocol

use crate::prelude::*;

/// `EFI_DRIVER_BINDING_PROTOCOL`
#[repr(C)]
pub struct DriverBinding {
    pub Supported: extern "efiapi" fn(*mut Self, Handle, *mut u8) -> Status,
    pub Start: extern "efiapi" fn(*mut Self, Handle, *mut u8) -> Status,
    pub Stop: extern "efiapi" fn(*mut Self, Handle, usize, Option<Handle>) -> Status,
    pub Version: u32,
    pub ImageHandle: Handle,
    pub DriverBindingHandle: Handle,
}

impl DriverBinding {
    pub const GUID: Guid = guid!("18a031ab-b443-4d1a-a5c0-0c09261e9f71");
}
