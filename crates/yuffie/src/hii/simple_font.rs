// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Simplified Font Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.2.7: Fonts
//!   - 33.3.2: Simplified Font Package

use super::*;

pub const GLYPH_HEIGHT: usize = 19;
pub const GLYPH_WIDTH: usize = 8;

/// `EFI_NARROW_GLYPH`
#[repr(C, packed)]
pub struct NarrowGlyph {
    pub UnicodeWeight: u16,
    pub Attributes: u8,
    pub GlyphCol1: [u8; GLYPH_HEIGHT],
}

/// `EFI_WIDE_GLYPH`
#[repr(C, packed)]
pub struct WideGlyph {
    pub UnicodeWeight: u16,
    pub Attributes: u8,
    pub GlyphCol1: [u8; GLYPH_HEIGHT],
    pub GlyphCol2: [u8; GLYPH_HEIGHT],
    _Pad: [u8; 3],
}

/// `EFI_HII_SIMPLE_FONT_PACKAGE_HDR`
#[repr(C, packed)]
pub struct SimpleFontPackageHeader {
    pub Header: PackageHeader,
    pub NumberOfNarrowGlyphs: u16,
    pub NumberOfWideGlyphs: u16,
}
