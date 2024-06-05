// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Driver family override protocol

use crate::prelude::*;

/// `EFI_DRIVER_FAMILY_OVERRIDE_PROTOCOL`
#[repr(C)]
pub struct DriverFamilyOverride {
    pub GetVersion: extern "efiapi" fn(*const Self) -> Status,
}

impl DriverFamilyOverride {
    pub const GUID: Guid = guid!("b1ee129e-da36-4181-91f8-04a4923766a7");
}
