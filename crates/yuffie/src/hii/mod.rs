// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Human Interface Infrastructure

pub mod ifr;

use crate::guid;

#[repr(transparent)]
pub struct PackageKind(u8);

impl PackageKind {
    pub const ALL: Self = Self(0x00);
    pub const GUID: Self = Self(0x01);
    pub const FORMS: Self = Self(0x02);
    pub const STRINGS: Self = Self(0x04);
    pub const FONTS: Self = Self(0x05);
    pub const IMAGES: Self = Self(0x06);
    pub const SIMPLE_FONTS: Self = Self(0x07);
    pub const DEVICE_PATH: Self = Self(0x08);
    pub const KEYBOARD_LAYOUT: Self = Self(0x09);
    pub const ANIMATIONS: Self = Self(0x0A);
    pub const END: Self = Self(0xDF);
    pub const SYSTEM_BEGIN: Self = Self(0xE0);
    pub const SYSTEM_END: Self = Self(0xFF);
}

impl From<u8> for PackageKind {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PackageKind> for u8 {
    fn from(value: PackageKind) -> Self {
        value.0
    }
}

#[repr(C)]
pub struct PackageHeader {
    pub kind_and_length: u32,
}

impl PackageHeader {
    pub fn kind(&self) -> PackageKind {
        PackageKind(((self.kind_and_length >> 24) & 0xFF) as u8)
    }

    pub fn length(&self) -> usize {
        (self.kind_and_length & 0xFF_FFFF) as usize
    }
}

#[repr(C)]
pub struct PackageListHeader {
    pub guid: guid::Guid,
    pub length: u32,
}
