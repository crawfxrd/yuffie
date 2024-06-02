// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Keyboard Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.2.4: Keyboard Layout
//!   - 33.3.9: Keyboard Package
//!   - 34.8.10: `EFI_HII_DATABASE_PROTOCOL.GetKeyboardLayout()`

/// `EFI_HII_KEYBOARD_LAYOUT`
#[repr(C, packed)]
pub struct KeyboardLayout {
    pub LayoutLength: u16,
    pub Guid: crate::Guid,
    pub LayoutDescriptorStringOffset: u32,
    pub DescriptorCount: u8,
}

/// `EFI_HII_KEYBOARD_PACKAGE_HDR`
#[repr(C, packed)]
pub struct KeyboardPackageHeader {
    pub Header: super::PackageHeader,
    pub LayoutCount: u16,
}
