// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Platform driver override protocol

use crate::prelude::*;

/// `EFI_PLATFORM_DRIVER_OVERRIDE_PROTOCOL`
#[repr(C)]
pub struct PlatformDriverOverride {
    pub GetDriver: extern "efiapi" fn(*mut Self, Handle, *mut Handle) -> Status,
    pub GetDriverPath: extern "efiapi" fn(*mut Self, Handle, *mut u8) -> Status,
    pub DriverLoaded: extern "efiapi" fn(*mut Self, Handle, *mut u8, Handle) -> Status,
}

impl PlatformDriverOverride {
    pub const GUID: Guid = guid!("6b30c738-a391-11d4-9a3b-0090273fc14d");
}
