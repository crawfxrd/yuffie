// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Tables

pub mod boot;
pub mod cfg;
pub mod runtime;
pub mod system;

pub use boot::BootServices;
pub use runtime::RuntimeServices;
pub use system::SystemTable;

/// Data struct that precedes all stable UEFI tables.
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    /// A 64-bit signature that identifies the type of table that follows.
    pub Signature: u64,
    /// The revision of the UEFI specification to which this table  conforms.
    pub Revision: u32,
    /// The size in bytes of the entire table including the header.
    pub Size: u32,
    /// The 32-bit CRC for the entire table.
    pub Crc32: u32,
    _Reserved: u32,
}

impl Header {
    /// A 64-bit signature that identifies the type of table that follows.
    pub fn signature(&self) -> u64 {
        self.Signature
    }

    /// The revision of the UEFI specification to which this table  conforms.
    pub fn revision(&self) -> u32 {
        self.Revision
    }

    /// The size in bytes of the entire table including the header.
    pub fn size(&self) -> u32 {
        self.Size
    }

    /// The 32-bit CRC for the entire table.
    pub fn crc32(&self) -> u32 {
        self.Crc32
    }
}
