// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # String Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.2.6: Strings
//!   - 33.3.6: String Package

use core::ffi;

use super::font::FontStyle;
use super::*;

pub const UEFI_CONFIG_LANG: &ffi::CStr = c"x-UEFI";
pub const UEFI_CONFIG_LANG_2: &ffi::CStr = c"x-i-UEFI";

/// `EFI_HII_STRING_PACKAGE_HDR`
#[repr(C, packed)]
pub struct StringPackageHeader<const N: usize = 0> {
    pub Header: PackageHeader,
    pub HdrSize: u32,
    pub StringInfoOffset: u32,
    pub LanguageWindow: [u16; 16],
    pub LanguageName: StringId,
    pub Language: [u8; N],
}

#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct StringBlockType(u8);

impl StringBlockType {
    pub const END: Self = Self(0x00);
    pub const STRING_SCSU: Self = Self(0x10);
    pub const STRING_SCSI_FONT: Self = Self(0x11);
    pub const STRINGS_SCSU: Self = Self(0x12);
    pub const STRINGS_SCSU_FONT: Self = Self(0x13);
    pub const STRING_UCS2: Self = Self(0x14);
    pub const STRING_UCS2_FONT: Self = Self(0x15);
    pub const STRINGS_UCS2: Self = Self(0x16);
    pub const STRINGS_UCS2_FONT: Self = Self(0x17);
    pub const DUPLICATE: Self = Self(0x20);
    pub const SKIP2: Self = Self(0x21);
    pub const SKIP1: Self = Self(0x22);
    pub const EXT1: Self = Self(0x30);
    pub const EXT2: Self = Self(0x31);
    pub const EXT4: Self = Self(0x32);
    pub const FONT: Self = Self(0x40);
}

/// `EFI_HII_STRING_BLOCK`
#[repr(C, packed)]
pub struct StringBlock {
    pub BlockType: StringBlockType,
}

/// `EFI_HII_SIBT_DUPLICATE_BLOCK`
#[repr(C, packed)]
pub struct SibtDuplicate {
    pub Header: StringBlock,
    pub StringId: StringId,
}

/// `EFI_HII_SIBT_END_BLOCK`
#[repr(C, packed)]
pub struct SibtEnd {
    pub Header: StringBlock,
}

/// `EFI_HII_SIBT_EXT1_BLOCK`
#[repr(C, packed)]
pub struct SibtExt1 {
    pub Header: StringBlock,
    pub BlockType2: StringBlockType,
    pub Length: u8,
}

/// `EFI_HII_SIBT_EXT2_BLOCK`
#[repr(C, packed)]
pub struct SibtExt2 {
    pub Header: StringBlock,
    pub BlockType2: StringBlockType,
    pub Length: u16,
}

/// `EFI_HII_SIBT_EXT4_BLOCK`
#[repr(C, packed)]
pub struct SibtExt4 {
    pub Header: StringBlock,
    pub BlockType2: StringBlockType,
    pub Length: u32,
}

/// `EFI_HII_SIBT_FONT_BLOCK`
#[repr(C, packed)]
pub struct SibtFont<const N: usize = 0> {
    pub Header: SibtExt2,
    pub FontId: u8,
    pub FontSize: u16,
    pub FontStyle: FontStyle,
    pub FontName: [u16; N],
}

/// `EFI_HII_SIBT_SKIP1_BLOCK`
#[repr(C, packed)]
pub struct SibtSkip1 {
    pub Header: StringBlock,
    pub SkipCount: u8,
}

/// `EFI_HII_SIBT_SKIP2_BLOCK`
#[repr(C, packed)]
pub struct SibtSkip2 {
    pub Header: StringBlock,
    pub SkipCount: u16,
}

/// `EFI_HII_SIBT_STRING_SCSU_BLOCK`
#[repr(C, packed)]
pub struct SibtStringScsu<const N: usize = 0> {
    pub Header: StringBlock,
    pub StringText: [u8; N],
}

/// `EFI_HII_SIBT_STRING_SCSU_FONT_BLOCK`
#[repr(C, packed)]
pub struct SibtStringScsuFont<const N: usize = 0> {
    pub Header: StringBlock,
    pub FontIdentifier: u8,
    pub StringText: [u8; N],
}

/// `EFI_HII_SIBT_STRINGS_SCSU_BLOCK`
#[repr(C, packed)]
pub struct SibtStringsScsu<const N: usize = 0> {
    pub Header: StringBlock,
    pub StringCount: u16,
    pub StringText: [u8; N],
}

/// `EFI_HII_SIBT_STRINGS_SCSU_FONT_BLOCK`
#[repr(C, packed)]
pub struct SibtStringsScsuFont<const N: usize = 0> {
    pub Header: StringBlock,
    pub FontIdentifier: u8,
    pub StringCount: u16,
    pub StringText: [u8; N],
}

/// `EFI_HII_SIBT_STRING_UCS2_BLOCK`
#[repr(C, packed)]
pub struct SibtStringUcs2<const N: usize = 0> {
    pub Header: StringBlock,
    pub StringText: [u16; N],
}

/// `EFI_HII_SIBT_STRING_UCS2_BLOCK`
#[repr(C, packed)]
pub struct SibtStringUcs2Font<const N: usize = 0> {
    pub Header: StringBlock,
    pub FontIdentifier: u8,
    pub StringText: [u16; N],
}

/// `EFI_HII_SIBT_STRINGS_UCS2_BLOCK`
#[repr(C, packed)]
pub struct SibtStringsUcs2<const N: usize = 0> {
    pub Header: StringBlock,
    pub StringCount: u16,
    pub StringText: [u16; N],
}

/// `EFI_HII_SIBT_STRINGS_UCS2_FONT_BLOCK`
#[repr(C, packed)]
pub struct SibtStringsUcs2Font<const N: usize = 0> {
    pub Header: StringBlock,
    pub FontIdentifier: u8,
    pub StringCount: u16,
    pub StringText: [u16; N],
}
