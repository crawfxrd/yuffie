// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Graphics output protocol

use crate::mem::PhysicalAddress;
use crate::prelude::*;

/// `EFI_PIXEL_BITMAP`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct PixelBitmap {
    pub RedMask: u32,
    pub GreenMask: u32,
    pub BlueMask: u32,
    pub ReservedMask: u32,
}

/// `EFI_GRAPHICS_PIXEL_FORMAT`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PixelFormat(u32);

impl PixelFormat {
    pub const RGB_RESERVED_8_BPP: Self = Self(0);
    pub const BGR_RESERVED_8_BPP: Self = Self(1);
    pub const BIT_MASK: Self = Self(2);
    pub const BLT_ONLY: Self = Self(3);
}

/// `EFI_GRAPHICS_OUTPUT_MODE_INFORMATION`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GraphicsOutputModeInfo {
    pub Version: u32,
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub PixelFormat: PixelFormat,
    pub PixelInformation: PixelBitmap,
    pub PixelsPerScanLine: u32,
}

/// `EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE`
#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GraphicsOutputMode {
    pub MaxMode: u32,
    pub Mode: u32,
    pub Info: *mut GraphicsOutputModeInfo,
    pub SizeOfInfo: usize,
    pub FrameBufferBase: PhysicalAddress,
    pub FrameBufferSize: usize,
}

/// `EFI_GRAPHICS_OUTPUT_BLT_PIXEL`
#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct GraphicsOutputBltPixel {
    pub Blue: u8,
    pub Green: u8,
    pub Red: u8,
    pub Reserved: u8,
}

/// `EFI_GRAPHICS_OUTPUT_BLT_OPERATION`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct GraphicsOutputBltOperation(u32);

impl GraphicsOutputBltOperation {
    pub const VIDEO_FILL: Self = Self(0);
    pub const VIDEO_TO_BLT_BUFFER: Self = Self(1);
    pub const BUFFER_TO_VIDEO: Self = Self(2);
    pub const VIDEO_TO_VIDEO: Self = Self(3);
}

/// `EFI_GRAPHICS_OUTPUT_PROTOCOL`
#[rustfmt::skip]
#[repr(C)]
pub struct GraphicsOutput {
    pub QueryMode: extern "efiapi" fn(*mut Self, u32, *mut usize, *mut *const GraphicsOutputModeInfo) -> Status,
    pub SetMode: extern "efiapi" fn(*mut Self, u32) -> Status,
    pub Blt: extern "efiapi" fn(*mut Self, *mut GraphicsOutputBltOperation, usize, usize, usize, usize, usize, usize, usize) -> Status,
    pub Mode: *mut GraphicsOutputMode,
}

impl GraphicsOutput {
    pub const GUID: Guid = guid!("9042a9de-23dc-4a38-96fb-7aded080516a");

    // TODO: QueryMode

    /// Set the video device into the specified mode and clears the visible
    /// portions of the output display to black.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The index is not supported by the device.
    pub fn set_mode(&mut self, index: u32) -> Result<()> {
        (self.SetMode)(self, index).into()
    }

    // TODO: Blt
}
