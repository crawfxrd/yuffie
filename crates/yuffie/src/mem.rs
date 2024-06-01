// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Memory data types

// 7.2.1 EFI_BOOT_SERVICES.AllocatePages()

#[repr(transparent)]
pub struct AllocateType(u32);
impl AllocateType {
    pub const ANY_PAGES: Self = Self(0);
    pub const MAX_ADDRESS: Self = Self(1);
    pub const ADDRESS: Self = Self(2);
}

#[repr(transparent)]
pub struct MemoryType(u32);

impl MemoryType {
    pub const RESERVED: Self = Self(0);
    pub const LOADER_CODE: Self = Self(1);
    pub const LOADER_DATA: Self = Self(2);
    pub const BOOTSERVICES_CODE: Self = Self(3);
    pub const BOOTSERVICES_DATA: Self = Self(4);
    pub const CONVENTIONAL: Self = Self(5);
    pub const UNUSABLE: Self = Self(6);
    pub const ACPI_RECLAIM: Self = Self(7);
    pub const ACPI_NVS: Self = Self(8);
    pub const MEMORY_MAPPED_IO: Self = Self(9);
    pub const MEMORY_MAPPED_IO_PORTSPACE: Self = Self(10);
    pub const PAL_CODE: Self = Self(11);
    pub const PERSISTENT: Self = Self(12);
    pub const UNACCEPTED: Self = Self(13);
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PhysicalAddress(u64);

impl From<u64> for PhysicalAddress {
    fn from(addr: u64) -> Self {
        Self(addr)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VirtualAddress(u64);

impl From<u64> for VirtualAddress {
    fn from(addr: u64) -> Self {
        Self(addr)
    }
}

#[repr(C)]
pub struct MemoryDescriptor {
    pub Type: u32,
    pub PhysicalStart: PhysicalAddress,
    pub VirtualStart: VirtualAddress,
    pub NumberOfPages: u64,
    pub Attribute: u64,
}
