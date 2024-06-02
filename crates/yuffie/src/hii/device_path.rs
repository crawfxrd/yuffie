// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Device Path Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.3.4: Device Path Package

#[repr(C, packed)]
pub struct DevicePathPackageHeader {
    pub Header: super::PackageHeader,
}
