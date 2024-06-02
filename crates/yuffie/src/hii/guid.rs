// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # GUID Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.3.5: GUID Package

use super::*;

/// `EFI_HII_GUID_PACKAGE_HDR`
#[repr(C, packed)]
pub struct GuidPackageHeader {
    pub Header: PackageHeader,
    pub Guid: crate::guid::Guid,
}
