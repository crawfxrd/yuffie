// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Pointer protocols

use crate::prelude::*;

// 12.5. Simple Pointer Protocol

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SimplePointerMode {
    pub ResolutionX: u64,
    pub ResolutionY: u64,
    pub ResolutionZ: u64,
    pub LeftButton: bool,
    pub RightButton: bool,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SimplePointerState {
    pub RelativeMovementX: i32,
    pub RelativeMovementY: i32,
    pub RelativeMovementZ: i32,
    pub LeftButton: bool,
    pub RightButton: bool,
}

#[repr(C)]
pub struct SimplePointer {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub GetState: extern "efiapi" fn(*mut Self, *mut SimplePointerState) -> Status,
    pub WaitForInput: Event,
    pub Mode: *mut SimplePointerMode,
}

impl SimplePointer {
    pub const GUID: Guid = guid!("31878c87-0b75-11d5-9a4f-0090273fc14d");
}

// 12.7. Absolute Pointer Protocol

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct AbsolutePointerMode {
    pub AbsoluteMinX: u64,
    pub AbsoluteMinY: u64,
    pub AbsoluteMinZ: u64,
    pub AbsoluteMaxX: u64,
    pub AbsoluteMaxY: u64,
    pub AbsoluteMaxZ: u64,
    pub Attributes: u32,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct AbsolutePointerState {
    pub CurrentX: u64,
    pub CurrentY: u64,
    pub CurrentZ: u64,
    pub ActiveButtons: u32,
}

#[repr(C)]
pub struct AbsolutePointer {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub GetState: extern "efiapi" fn(*mut Self, *mut AbsolutePointerState) -> Status,
    pub WaitForInput: Event,
    pub Mode: *mut AbsolutePointerMode,
}

impl AbsolutePointer {
    pub const GUID: Guid = guid!("8d59d32b-c655-4ae9-9b15-f25904992a43");
}
