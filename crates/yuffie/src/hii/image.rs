// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Image Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.2.8: Images
//!   - 33.3.7: Image Package

use super::*;

/// `EFI_HII_IMAGE_PACKAGE_HDR`
#[repr(C, packed)]
pub struct ImagePackageHeader {
    pub Headre: PackageHeader,
    pub ImageInfoOffset: u32,
    pub PaletteInfoOffset: u32,
}

#[repr(transparent)]
pub struct ImageBlockType(u8);

impl ImageBlockType {
    pub const END: Self = Self(0x00);
    pub const IMAGE_1BIT: Self = Self(0x10);
    pub const IMAGE_1BIT_TRANS: Self = Self(0x11);
    pub const IMAGE_4BIT: Self = Self(0x12);
    pub const IMAGE_4BIT_TRANS: Self = Self(0x13);
    pub const IMAGE_8BIT: Self = Self(0x14);
    pub const IMAGE_8BIT_TRANS: Self = Self(0x15);
    pub const IMAGE_24BIT: Self = Self(0x16);
    pub const IMAGE_24BIT_TRANS: Self = Self(0x17);
    pub const IMAGE_JPEG: Self = Self(0x18);
    pub const IMAGE_PNG: Self = Self(0x19);
    pub const DUPLICATE: Self = Self(0x20);
    pub const SKIP2: Self = Self(0x21);
    pub const SKIP1: Self = Self(0x22);
    pub const EXT1: Self = Self(0x30);
    pub const EXT2: Self = Self(0x31);
    pub const EXT4: Self = Self(0x32);
}

/// `EFI_HII_IMAGE_BLOCK`
#[repr(C, packed)]
pub struct ImageBlock {
    pub BlockType: ImageBlockType,
}

/// `EFI_HII_IIBT_END_BLOCK`
#[repr(C, packed)]
pub struct IibtEnd {
    pub Header: ImageBlock,
}

/// `EFI_HII_IIBT_EXT1_BLOCK`
#[repr(C, packed)]
pub struct IibtExt1 {
    pub Header: ImageBlock,
    pub BlockType2: ImageBlockType,
    pub Length: u8,
}

/// `EFI_HII_IIBT_EXT2_BLOCK`
#[repr(C, packed)]
pub struct IibtExt2 {
    pub Header: ImageBlock,
    pub BlockType2: ImageBlockType,
    pub Length: u16,
}

/// `EFI_HII_IIBT_EXT4_BLOCK`
#[repr(C, packed)]
pub struct IibtExt4 {
    pub Header: ImageBlock,
    pub BlockType2: ImageBlockType,
    pub Length: u32,
}

/// `EFI_HII_IIBT_IMAGE_1BIT_BASE`
#[repr(C, packed)]
pub struct IibtImage1BitBase<const N: usize = 0> {
    pub Width: u16,
    pub Height: u16,
    pub Data: [u8; N],
}

/// `EFI_HII_IIBT_IMAGE_1BIT_BLOCK`
#[repr(C, packed)]
pub struct IibtImage1Bit {
    pub Header: ImageBlock,
    pub PaletteIndex: u8,
    pub Bitmap: IibtImage1BitBase,
}

/// `EFI_HII_IIBT_IMAGE_1BIT_TRANS_BLOCK`
#[repr(C, packed)]
pub struct IibtImage1BitTrans {
    pub Header: ImageBlock,
    pub PaletteIndex: u8,
    pub Bitmap: IibtImage1BitBase,
}

/// `EFI_HII_RGB_PIXEL`
#[repr(C, packed)]
pub struct RgbPixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

/// `EFI_HII_IIBT_IMAGE_24BIT_BASE`
#[repr(C, packed)]
pub struct IibtImage24BitBase<const N: usize = 0> {
    pub Width: u16,
    pub Height: u16,
    pub Bitmap: [RgbPixel; N],
}

/// `EFI_HII_IIBT_IMAGE_24BIT_BLOCK`
#[repr(C, packed)]
pub struct IibtImage24Bit {
    pub Header: ImageBlock,
    pub Bitmap: IibtImage24BitBase,
}

/// `EFI_HII_IIBT_IMAGE_24BIT_TRANS_BLOCK`
#[repr(C, packed)]
pub struct IibtImage24BitTrans {
    pub Header: ImageBlock,
    pub Bitmap: IibtImage24BitBase,
}

/// `EFI_HII_IIBT_IMAGE_4BIT_BASE`
#[repr(C, packed)]
pub struct IibtImage4BitBase<const N: usize = 0> {
    pub Width: u16,
    pub Height: u16,
    pub Data: [u8; N],
}

/// `EFI_HII_IIBT_IMAGE_4BIT_BLOCK`
#[repr(C, packed)]
pub struct IibtImage4Bit {
    pub Header: ImageBlock,
    pub PaletteIndex: u8,
    pub Bitmap: IibtImage4BitBase,
}

/// `EFI_HII_IIBT_IMAGE_4BIT_TRANS_BLOCK`
#[repr(C, packed)]
pub struct IibtImage4BitTrans {
    pub Header: ImageBlock,
    pub PaletteIndex: u8,
    pub Bitmap: IibtImage4BitBase,
}

/// `EFI_HII_IIBT_IMAGE_8BIT_BASE`
#[repr(C, packed)]
pub struct IibtImage8BitBase<const N: usize = 0> {
    pub Width: u16,
    pub Height: u16,
    pub Data: [u8; N],
}

/// `EFI_HII_IIBT_IMAGE_8BIT_BLOCK`
#[repr(C, packed)]
pub struct IibtImage8Bit {
    pub Header: ImageBlock,
    pub PaletteIndex: u8,
    pub Bitmap: IibtImage8BitBase,
}

/// `EFI_HII_IIBT_IMAGE_8BIT_TRANS_BLOCK`
#[repr(C, packed)]
pub struct IibtImage8BitTrans {
    pub Header: ImageBlock,
    pub PaletteIndex: u8,
    pub Bitmap: IibtImage8BitBase,
}

/// `EFI_HII_IIBT_DUPLICATE_BLOCK`
#[repr(C, packed)]
pub struct IibtDuplicate {
    pub Header: ImageBlock,
    pub ImageId: ImageId,
}

/// `EFI_HII_IIBT_JPEG_BLOCK`
#[repr(C, packed)]
pub struct IibtJpeg<const N: usize = 0> {
    pub Header: ImageBlock,
    pub Size: u32,
    pub Data: [u8; N],
}

/// `EFI_HII_IIBT_PNG_BLOCK`
#[repr(C, packed)]
pub struct IibtPng<const N: usize = 0> {
    pub Header: ImageBlock,
    pub Size: u32,
    pub Data: [u8; N],
}

/// `EFI_HII_IIBT_SKIP1_BLOCK`
#[repr(C, packed)]
pub struct IibtSkip1 {
    pub Header: ImageBlock,
    pub SkipCount: u8,
}

/// `EFI_HII_IIBT_SKIP2_BLOCK`
#[repr(C, packed)]
pub struct IibtSkip2 {
    pub Header: ImageBlock,
    pub SkipCount: u16,
}

/// `EFI_HII_IMAGE_PALETTE_INFO_HEADER`
#[repr(C, packed)]
pub struct PaletteInfoHeader {
    pub PaletteCount: u16,
}

/// `EFI_HII_IMAGE_PALETTE_INFO`
#[repr(C, packed)]
pub struct PaletteInfo<const N: usize = 0> {
    pub PaletteSize: u16,
    pub PaletteValue: [RgbPixel; N],
}
