// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Adapter information protocol

use crate::prelude::*;

/// `EFI_ADAPTER_INFORMATION_PROTOCOL`
#[rustfmt::skip]
#[repr(C)]
pub struct AdapterInformation {
    pub GetInformation: extern "efiapi" fn(*mut Self, *const Guid, *mut *mut u8, *mut usize) -> Status,
    pub SetInformation: extern "efiapi" fn(*mut Self, *const Guid, *mut u8, usize) -> Status,
    pub GetSuppportedTypes: extern "efiapi" fn(*mut Self, *mut *mut Guid, *mut usize) -> Status,
}

impl AdapterInformation {
    pub const GUID: Guid = guid!("e5dd1403-d622-c24e-8488-c71b17f5e802");
}
