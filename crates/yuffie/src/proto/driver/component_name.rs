// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Component name protocol

use crate::prelude::*;

/// `EFI_COMPONENT_NAME2_PROTOCOL`
#[rustfmt::skip]
#[repr(C)]
pub struct ComponentName2 {
    pub GetDriverName: extern "efiapi" fn(*mut Self, *const u8, *mut *mut u16) -> Status,
    pub GetControllerName: extern "efiapi" fn(*mut Self, Handle, Option<Handle>, *const u8, *mut *mut u16) -> Status,
    pub SupportedLanguages: extern "efiapi" fn() -> Status,
}

impl ComponentName2 {
    pub const GUID: Guid = guid!("6a7a5cff-e8d9-4f70-bada-75ab3025ce14");
}
