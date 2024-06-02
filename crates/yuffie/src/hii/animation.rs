// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Animations Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.3.10: Animations Package

use self::image::RgbPixel;
use super::*;

/// `EFI_HII_ANIMATION_PACKAGE_HDR`
#[repr(C, packed)]
pub struct AnimationPackageHeader {
    // NOTE: UEFI 2.10 incorrectly uses non-existent `EFI_HII_ANIMATION_PACKAGE`
    pub Header: PackageHeader,
    pub AnimationInfoOffset: u32,
}

#[repr(transparent)]
pub struct AnimationBlockType(u8);

impl AnimationBlockType {
    pub const END: Self = Self(0x00);
    pub const OVERLAY_IMAGES: Self = Self(0x10);
    pub const CLEAR_IMAGES: Self = Self(0x11);
    pub const RESTORE_SCRN: Self = Self(0x12);
    pub const OVERLAY_IMAGES_LOOP: Self = Self(0x18);
    pub const CLEAR_IMAGES_LOOP: Self = Self(0x19);
    pub const RESTORE_SCRN_LOOP: Self = Self(0x1A);
    pub const DUPLICATE: Self = Self(0x20);
    pub const SKIP2: Self = Self(0x21);
    pub const SKIP1: Self = Self(0x22);
    pub const EXT1: Self = Self(0x30);
    pub const EXT2: Self = Self(0x31);
    pub const EXT4: Self = Self(0x32);
}

/// `EFI_HII_ANIMATION_BLOCK`
#[repr(C, packed)]
pub struct AnimationBlock {
    pub BlockType: AnimationBlockType,
}

/// `EFI_HII_AIBT_EXT1_BLOCK`
#[repr(C, packed)]
pub struct AibtExt1 {
    pub Header: AnimationBlock,
    pub BlockType2: AnimationBlockType,
    pub Length: u8,
}

/// `EFI_HII_AIBT_EXT2_BLOCK`
#[repr(C, packed)]
pub struct AibtExt2 {
    pub Header: AnimationBlock,
    pub BlockType2: AnimationBlockType,
    pub Length: u8,
}

/// `EFI_HII_AIBT_EXT4_BLOCK`
#[repr(C, packed)]
pub struct AibtExt4 {
    pub Header: AnimationBlock,
    pub BlockType2: AnimationBlockType,
    pub Length: u8,
}

/// `EFI_HII_ANIMATION_CELL`
#[repr(C, packed)]
pub struct AnimationCell {
    pub OffsetX: u16,
    pub OffsetY: u16,
    pub ImageId: ImageId,
    pub Delay: u16,
}

/// `EFI_HII_AIBT_OVERLAY_IMAGES_BLOCK`
#[repr(C, packed)]
pub struct AibtOverlayImages<const N: usize = 0> {
    pub DftImageId: ImageId,
    pub Width: u16,
    pub Height: u16,
    pub CellCount: u16,
    pub AnimationCell: [AnimationCell; N],
}

/// `EFI_HII_AIBT_CLEAR_IMAGES_BLOCK`
#[repr(C, packed)]
pub struct AibtClearImages<const N: usize = 0> {
    pub DftImageId: ImageId,
    pub Width: u16,
    pub Height: u16,
    pub CellCount: u16,
    pub BackgndColor: RgbPixel,
    pub AnimationCell: [AnimationCell; N],
}

/// `EFI_HII_AIBT_RESTORE_SCRN_BLOCK`
#[repr(C, packed)]
pub struct AibtRestoreScreen<const N: usize = 0> {
    pub DftImageId: ImageId,
    pub Width: u16,
    pub Height: u16,
    pub CellCount: u16,
    pub AnimationCell: [AnimationCell; N],
}

/// `EFI_HII_AIBT_OVERLAY_IMAGES_LOOP_BLOCK`
pub type AibtOverlayImagesLoop = AibtOverlayImages;

/// `EFI_HII_AIBT_CLEAR_IMAGES_LOOP_BLOCK`
pub type AibtClearImagesLoop = AibtClearImages;

/// `EFI_HII_AIBT_RESTORE_SCRN_LOOP_BLOCK`
pub type AibtRestoreScreenLoop = AibtRestoreScreen;

/// `EFI_HII_AIBT_DUPLICATE_BLOCK`
#[repr(C, packed)]
pub struct AibtDuplicate {
    pub AnimationId: AnimationId,
}

/// `EFI_HII_AIBT_SKIP1_BLOCK`
#[repr(C, packed)]
pub struct AibtSkip1 {
    pub SkipCount: u8,
}

/// `EFI_HII_AIBT_SKIP2_BLOCK`
#[repr(C, packed)]
pub struct AibtSkip2 {
    pub SkipCount: u16,
}
