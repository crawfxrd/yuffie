// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Serial I/O protocols

use core::ops;

use crate::prelude::*;

/// `EFI_PARITY_TYPE`
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

/// `EFI_STOP_BITS_TYPE`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct StopBitsType(u32);

impl StopBitsType {
    pub const DEFAULT: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const ONE_FIVE: Self = Self(2);
    pub const TWO: Self = Self(3);
}

/// `EFI_SERIAL_IO_MODE`
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

#[derive(Debug, Default, Eq, PartialEq)]
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

impl ops::BitAnd for ControlBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitOr for ControlBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// `EFI_SERIAL_IO_PROTOCOL`
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

    /// Resets the serial device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    pub fn reset(&mut self) -> Result<()> {
        (self.Reset)(self).into()
    }

    /// Sets the baud rate, receive FIFO depth, transmit/receive timeout,
    /// parity, data bits, and stop bits on the serial device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `INVALID_PARAMETER`: One or more of the attributes has an unsupported
    ///   value.
    pub fn set_attributes(
        &mut self,
        baud: u64,
        fifo_depth: u32,
        timeout: u32,
        parity: ParityType,
        data_bits: u8,
        stop_bits: StopBitsType,
    ) -> Result<()> {
        (self.SetAttributes)(self, baud, fifo_depth, timeout, parity, data_bits, stop_bits).into()
    }

    /// Sets the control bits on the serial device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The serial device does not support this operation.
    pub fn set_control(&mut self, control: ControlBits) -> Result<()> {
        (self.SetControl)(self, control).into()
    }

    /// Gets the status of the control bits on the serial device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    pub fn get_control(&mut self) -> Result<ControlBits> {
        let mut control = ControlBits::default();
        let status = (self.GetControl)(self, &mut control);

        match status {
            Status::SUCCESS => Ok(control),
            err => Err(err),
        }
    }

    /// Writes data to the serial device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `TIMEOUT`: The operation was stopped due to a timeout.
    pub fn write(&mut self, buffer: &[u8]) -> Result<usize> {
        let mut size = buffer.len();
        let status = (self.Write)(self, &mut size, buffer.as_ptr());

        match status {
            Status::SUCCESS => Ok(size),
            err => Err(err),
        }
    }

    /// Reads data from the serial device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `TIMEOUT`: The operation was stopped due to a timeout or overrun.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let mut size = buffer.len();
        let status = (self.Read)(self, &mut size, buffer.as_mut_ptr());

        match status {
            Status::SUCCESS => Ok(size),
            err => Err(err),
        }
    }
}
