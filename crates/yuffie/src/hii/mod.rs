// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Human Interface Infrastructure
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33: Human Interface Infrastructure Overview

pub mod animation;
pub mod device_path;
pub mod font;
pub mod forms;
pub mod guid;
pub mod image;
pub mod keyboard;
pub mod simple_font;
pub mod string;

pub type QuestionId = u16;
pub type ImageId = u16;
pub type StringId = u16;
pub type FormId = u16;
pub type VarStoreId = u16;
pub type AnimationId = u16;
pub type DefaultId = u16;

/// Package type.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

/// `EFI_HII_PACKAGE_HEADER`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PackageHeader {
    pub kind_and_length: u32,
}

impl PackageHeader {
    /// The package type.
    pub fn kind(&self) -> PackageKind {
        PackageKind(((self.kind_and_length >> 24) & 0xFF) as u8)
    }

    /// The size of the package in bytes.
    pub fn length(&self) -> usize {
        (self.kind_and_length & 0xFF_FFFF) as usize
    }
}

/// `EFI_HII_PACKAGE_LIST_HEADER`
#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PackageListHeader {
    /// The GUID applied to the list of packages which follows.
    pub guid: crate::Guid,
    /// The size of of the package list in bytes, including the header.
    pub length: u32,
}
