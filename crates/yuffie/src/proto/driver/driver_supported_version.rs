// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Driver supported UEFI version protocol

use crate::prelude::*;

/// `EFI_DRIVER_SUPPORTED_EFI_VERSION_PROTOCOL`
#[repr(C)]
pub struct DriverSupportedUefiVersion {
    pub Length: u32,
    pub FirmwareVersion: u32,
}

impl DriverSupportedUefiVersion {
    pub const GUID: Guid = guid!("5c198761-16a8-4e69-972c-89d67954f81d");
}
