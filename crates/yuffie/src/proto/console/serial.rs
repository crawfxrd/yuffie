// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Serial I/O protocols

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ParityType(u32);

impl ParityType {
    pub const DEFAULT: Self = Self(0);
    pub const NONE: Self = Self(1);
    pub const EVEN: Self = Self(2);
    pub const ODD: Self = Self(3);
    pub const MARK: Self = Self(4);
    pub const SPACE: Self = Self(5);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct StopBitsType(u32);

impl StopBitsType {
    pub const DEFAULT: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const ONE_FIVE: Self = Self(2);
    pub const TWO: Self = Self(3);
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SerialIoMode {
    pub ControlMask: u32,
    pub Timeout: u32,
    pub BaudRate: u64,
    pub ReceiveFifoDepth: u32,
    pub DataBits: u32,
    pub Parity: ParityType,
    pub StopBits: StopBitsType,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ControlBits(u32);

impl ControlBits {
    pub const DATA_TERMINAL_READY: Self = Self(1 << 0);
    pub const REQUEST_TO_SEND: Self = Self(1 << 1);
    pub const CLEAR_TO_SEND: Self = Self(1 << 4);
    pub const DATA_SET_READY: Self = Self(1 << 5);
    pub const RING_INDICATE: Self = Self(1 << 6);
    pub const CARRIER_DETECT: Self = Self(1 << 7);
    pub const INPUT_BUFFER_EMPTY: Self = Self(1 << 8);
    pub const OUTPUT_BUFFER_EMPTY: Self = Self(1 << 9);
    pub const HARDWARE_LOOPBACK_ENABLE: Self = Self(1 << 12);
    pub const SOFTWARE_LOOPBACK_ENABLE: Self = Self(1 << 13);
    pub const HARDWARE_FLOW_CONTROL_ENABLE: Self = Self(1 << 14);
}

#[rustfmt::skip]
#[repr(C)]
pub struct SerialIo {
    pub Revision: u32,
    pub Reset: extern "efiapi" fn(*mut Self) -> Status,
    pub SetAttributes: extern "efiapi" fn(*mut Self, u64, u32, u32, ParityType, u8, StopBitsType) -> Status,
    pub SetControl: extern "efiapi" fn(*mut Self, ControlBits) -> Status,
    pub GetControl: extern "efiapi" fn(*mut Self, *mut ControlBits) -> Status,
    pub Write: extern "efiapi" fn(*mut Self, *mut usize, *const u8) -> Status,
    pub Read: extern "efiapi" fn(*mut Self, *mut usize, *mut u8) -> Status,
    pub Mode: *mut SerialIoMode,
    pub DeviceTypeGuid: *const Guid,
}

impl SerialIo {
    pub const GUID: Guid = guid!("bb25cf6f-f1d4-11d2-9a0c-0090273fc1fd");
}
