// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! EDID protocols

use crate::prelude::*;

/// `EFI_EDID_DISCOVERED_PROTOCOL`
#[repr(C)]
pub struct EdidDiscovered {
    pub SizeOfEdid: u32,
    pub Edid: *mut u8,
}

impl EdidDiscovered {
    pub const GUID: Guid = guid!("1c0c34f6-d380-41fa-a049-8ad06c1a66aa");
}

/// `EFI_EDID_ACTIVE_PROTOCOL`
#[repr(C)]
pub struct EdidActive {
    pub SizeOfEdid: u32,
    pub Edid: *mut u8,
}

impl EdidActive {
    pub const GUID: Guid = guid!("bd8c1056-9f36-44ec-92a8-a6337f817986");
}

/// `EFI_EDID_OVERRIDE_PROTOCOL`
#[rustfmt::skip]
#[repr(C)]
pub struct EdidOverride {
    pub GetEdid: extern "efiapi" fn(*mut Self, *const Handle, *mut u32, *mut usize, *mut *mut u8) -> Status,
}

impl EdidOverride {
    pub const GUID: Guid = guid!("48ecb431-fb72-45c0-a922-f458fe040bd5");
}
