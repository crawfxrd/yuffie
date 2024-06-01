// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Configuration Table and Properties Table

use crate::guid::Guid;

#[repr(C)]
pub struct ConfigurationTable {
    pub VendorGuid: Guid,
    pub VendorTable: usize,
}
