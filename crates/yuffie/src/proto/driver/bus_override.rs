// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Bus-specific driver override protocol

use crate::prelude::*;

/// `EFI_BUS_SPECIFIC_DRIVER_OVERRIDE_PROTOCOL`
#[repr(C)]
pub struct BusSpecificDriverOverride {
    pub GetDriver: extern "efiapi" fn(*mut Self, *mut Handle) -> Status,
}

impl BusSpecificDriverOverride {
    pub const GUID: Guid = guid!("3bc1b285-8a15-4a82-aabf-4d7d13fb3265");
}
