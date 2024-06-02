// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Font Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.2.7: Fonts
//!   - 33.3.3: Font Package

use super::*;

/// `EFI_HII_FONT_STYLE`
#[repr(transparent)]
pub struct FontStyle(u32);

impl FontStyle {
    pub const NORMAL: Self = Self(0);
    pub const BOLD: Self = Self(1 << 0);
    pub const ITALIC: Self = Self(1 << 1);
    pub const EMBOSS: Self = Self(1 << 16);
    pub const OUTLINE: Self = Self(1 << 17);
    pub const SHADOW: Self = Self(1 << 18);
    pub const UNDERLINE: Self = Self(1 << 19);
    pub const DBL_UNDER: Self = Self(1 << 20);
}

/// `EFI_HII_GLYPH_INFO`
#[repr(C, packed)]
pub struct GlyphInfo {
    pub Width: u16,
    pub Height: u16,
    pub OffsetX: i16,
    pub OffsetY: i16,
    pub AdvanceX: i16,
}

/// `EFI_HII_FONT_PACKAGE_HDR`
#[repr(C, packed)]
pub struct FontPackageHeader<const N: usize = 0> {
    pub Header: PackageHeader,
    pub HdrSize: u32,
    pub GlyphBlockOffset: u32,
    pub Cell: GlyphInfo,
    pub FontStyle: FontStyle,
    pub FontFamily: [u16; N],
}

#[repr(transparent)]
pub struct GlyphBlockType(u8);

impl GlyphBlockType {
    pub const END: Self = Self(0x00);
    pub const GLYPH: Self = Self(0x10);
    pub const GLYPHS: Self = Self(0x11);
    pub const GLYPH_DEFAULT: Self = Self(0x12);
    pub const GLYPHS_DEFAULT: Self = Self(0x13);
    pub const GLYPH_VARIABILITY: Self = Self(0x14);
    pub const DUPLICATE: Self = Self(0x20);
    pub const SKIP2: Self = Self(0x21);
    pub const SKIP1: Self = Self(0x22);
    pub const DEFAULTS: Self = Self(0x23);
    pub const EXT1: Self = Self(0x30);
    pub const EXT2: Self = Self(0x31);
    pub const EXT4: Self = Self(0x32);
}

/// `EFI_HII_GLYPH_BLOCK`
#[repr(C, packed)]
pub struct GlyphBlock {
    pub BlockType: GlyphBlockType,
}

/// `EFI_HII_GIBT_DEFAULTS_BLOCK`
#[repr(C, packed)]
pub struct GibtDefaults {
    pub Header: GlyphBlock,
    pub Cell: GlyphInfo,
}

/// `EFI_HII_GIBT_DUPLICATE_BLOCK`
#[repr(C, packed)]
pub struct GibtDuplicate {
    pub Header: GlyphBlock,
    pub CharValue: u16,
}

/// `EFI_HII_GIBT_END_BLOCK`
#[repr(C, packed)]
pub struct GibtEnd {
    pub Header: GlyphBlock,
}

/// `EFI_HII_GIBT_EXT1_BLOCK`
#[repr(C, packed)]
pub struct GibtExt1 {
    pub Header: GlyphBlock,
    pub BlockType2: GlyphBlockType,
    pub Length: u8,
}

/// `EFI_HII_GIBT_EXT2_BLOCK`
#[repr(C, packed)]
pub struct GibtExt2 {
    pub Header: GlyphBlock,
    pub BlockType2: GlyphBlockType,
    pub Length: u16,
}

/// `EFI_HII_GIBT_EXT4_BLOCK`
#[repr(C, packed)]
pub struct GibtExt4 {
    pub Header: GlyphBlock,
    pub BlockType2: GlyphBlockType,
    pub Length: u32,
}

/// `EFI_HII_GIBT_GLYPH_BLOCK`
#[repr(C, packed)]
pub struct GibtGlyph<const N: usize = 0> {
    pub Header: GlyphBlock,
    pub Cell: GlyphInfo,
    pub BitmapData: [u8; N],
}

/// `EFI_HII_GIBT_GLYPHS_BLOCK`
#[repr(C, packed)]
pub struct GibtGlyphs<const N: usize = 0> {
    pub Header: GlyphBlock,
    pub Cell: GlyphInfo,
    pub Count: u16,
    pub BitmapData: [u8; N],
}

/// `EFI_HII_GIBT_GLYPH_DEFAULT_BLOCK`
#[repr(C, packed)]
pub struct GibtGlyphDefault<const N: usize = 0> {
    pub Header: GlyphBlock,
    pub BitmapData: [u8; N],
}

/// `EFI_HII_GIBT_GLYPHS_DEFAULT_BLOCK`
#[repr(C, packed)]
pub struct GibtGlyphsDefault<const N: usize = 0> {
    pub Header: GlyphBlock,
    pub Count: u16,
    pub BitmapData: [u8; N],
}

/// `EFI_HII_GIBT_SKIP2_BLOCK`
#[repr(C, packed)]
pub struct GibtSkip2 {
    pub Header: GlyphBlock,
    pub SkipCount: u16,
}

/// `EFI_HII_GIBT_SKIP1_BLOCK`
#[repr(C, packed)]
pub struct GibtSkip1 {
    pub Header: GlyphBlock,
    pub SkipCount: u8,
}

/// `EFI_HII_GIBT_VARIABILITY_BLOCK`
#[repr(C, packed)]
pub struct GibtVariability<const N: usize = 0> {
    pub Header: GlyphBlock,
    pub Cell: GlyphInfo,
    pub GlyphPackInBits: u8,
    pub GitmapData: [u8; N],
}
