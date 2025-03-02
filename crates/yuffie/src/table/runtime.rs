// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Runtime Services Table
//!
//! Wrappers to the UEFI Runtime Services table.
//!
//! ## References
//!
//! - [UEFI Specification, Version 2.10][UEFI Spec]
//!   - 4.5: EFI Runtime Services Table
//!   - 8: Services - Runtime Services
//!
//! [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_10_Aug29.pdf

use super::Header;
use crate::mem::MemoryDescriptor;
use crate::mem::PhysicalAddress;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct CapsuleFlags(u32);

impl CapsuleFlags {
    pub const PERSIST_ACROSS_RESET: Self = Self(1 << 16);
    pub const POPULATE_SYSTEM_TABLE: Self = Self(1 << 17);
    pub const INITIATE_RESET: Self = Self(1 << 18);
}

#[derive(Debug)]
#[repr(C)]
pub struct CapsuleHeader {
    pub CapsuleGuid: Guid,
    pub HeaderSize: u32,
    pub Flags: CapsuleFlags,
    pub CapsuleImageSize: u32,
}

/// System reset type
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ResetType(u32);

impl ResetType {
    /// Trigger a system-wide reset
    pub const COLD: Self = Self(0);
    /// Trigger a system-wide re-initialization
    pub const WARM: Self = Self(1);
    /// Trigger a system power-off (ACPI S5 or G3 state)
    pub const SHUTDOWN: Self = Self(2);
    /// Trigger a system-wide reset defined by an associated GUID
    pub const PLATFORM_SPECIFIC: Self = Self(3);
}

/// Time information
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Time {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
    pub Hour: u8,
    pub Minute: u8,
    pub Second: u8,
    _Pad1: u8,
    pub Nanosecond: u32,
    pub TimeZone: i16,
    pub Daylight: u8,
    _Pad2: u8,
}

/// Capabilities of the real-time clock (RTC).
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TimeCapabilities {
    pub Resolution: u32,
    pub Accuracy: u32,
    pub SetsToZero: bool,
}

#[repr(transparent)]
pub struct VariableAttributes(u32);

impl VariableAttributes {
    pub const NON_VOLATILE: Self = Self(1 << 0);
    pub const BOOTSERVICE_ACCESS: Self = Self(1 << 1);
    pub const RUNTIME_ACCESS: Self = Self(1 << 2);
    pub const HARDWARE_ERROR_RECORD: Self = Self(1 << 3);
    pub const AUTHENTICATED_WRITE_ACCESS: Self = Self(1 << 4);
    pub const TIME_BASED_AUTHENTICATED_WRITE_ACCESS: Self = Self(1 << 5);
    pub const APPEND_WRITE: Self = Self(1 << 6);
    pub const ENHANCED_AUTHENTICATED_ACCESS: Self = Self(1 << 7);
}

/// The UEFI Runtime Services table
#[rustfmt::skip]
#[repr(C)]
pub struct RuntimeServices {
    pub Hdr: Header,

    // Time Services
    pub GetTime: extern "efiapi" fn(*mut Time, *mut TimeCapabilities) -> Status,
    pub SetTime: extern "efiapi" fn(*const Time) -> Status,
    pub GetWakeupTime: extern "efiapi" fn(*mut bool, *mut bool, *mut Time) -> Status,
    pub SetWakeupTime: extern "efiapi" fn(bool, *const Time) -> Status,

    // Virtual Memory Services
    pub SetVirtualAddressMap: extern "efiapi" fn(usize, usize, u32, *const MemoryDescriptor) -> Status,
    pub ConvertPointer: extern "efiapi" fn(usize, *mut *const u8) -> Status,

    // Variable Services
    pub GetVariable: extern "efiapi" fn(*const u16, *const Guid, *mut VariableAttributes, *mut usize, *mut u8) -> Status,
    pub GetNextVariableName: extern "efiapi" fn(*mut usize, *mut u16, *mut Guid) -> Status,
    pub SetVariable: extern "efiapi" fn(*const u16, *const Guid, VariableAttributes, usize, *const u8) -> Status,

    // Misc Services
    pub GetNextHighMonotonicCount: extern "efiapi" fn(*mut u32) -> Status,
    pub ResetSystem: extern "efiapi" fn(ResetType, Status, usize, *const u8),

    // UEFI 2.0 Capsule Services
    pub UpdateCapsule: extern "efiapi" fn(*const *const CapsuleHeader, usize, *const PhysicalAddress) -> Status,
    pub QueryCapsuleCapabilities: extern "efiapi" fn() -> Status,

    // Misc UEFI 2.0 Service
    pub QueryVariableInfo: extern "efiapi" fn(VariableAttributes, *mut u64, *mut u64, *mut u64) -> Status,
}

impl RuntimeServices {
    pub fn header(&self) -> &Header {
        &self.Hdr
    }

    /// Gets the current time and date information.
    ///
    /// # Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn get_time(&self) -> Result<Time> {
        let mut time = Time::default();
        let status = (self.GetTime)(&mut time, core::ptr::null_mut());

        match status {
            Status::SUCCESS => Ok(time),
            e => Err(e),
        }
    }

    /// Gets the time-keeping capabilities of the hardware platform.
    ///
    /// # Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn get_time_capabilities(&self) -> Result<TimeCapabilities> {
        let mut time = Time::default();
        let mut caps = TimeCapabilities::default();
        let status = (self.GetTime)(&mut time, &mut caps);

        match status {
            Status::SUCCESS => Ok(caps),
            e => Err(e),
        }
    }

    /// Sets the current local time and date information.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: A time field is out of range.
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn set_time(&mut self, time: &Time) -> Result<()> {
        (self.SetTime)(time).into()
    }

    /// Gets the current wakeup alarm clock setting.
    ///
    /// # Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn get_wakeup_time(&self) -> Result<(bool, bool, Time)> {
        let mut enabled = false;
        let mut pending = false;
        let mut time = Time::default();
        let status = (self.GetWakeupTime)(&mut enabled, &mut pending, &mut time);

        match status {
            Status::SUCCESS => Ok((enabled, pending, time)),
            e => Err(e),
        }
    }

    /// Sets the system wakeup alarm clock time.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: A time field is out of range.
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn set_wakeup_time(&mut self, time: &Time) -> Result<()> {
        (self.SetWakeupTime)(true, time).into()
    }

    /// Disables the system wakeup alarm clock.
    ///
    /// # Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn disable_wakeup_time(&mut self) -> Result<()> {
        (self.SetWakeupTime)(false, core::ptr::null()).into()
    }

    /// Changes the runtime addressing mode from physical to virtual.
    ///
    /// # Errors
    ///
    /// - `UNSUPPORTED`: Firmware is not at run-time, or firmware is already in
    ///   virtual address mapped mode.
    /// - `INVALID_PARAMETER`: `desc_size` or `desc_version` is invalid.
    /// - `NO_MAPPING`: A virtual address was not supplied for a range in the
    ///   memory map that requires a mapping.
    /// - `NOT_FOUND`: A virtual address was supplied for an address that is not
    ///   found in the memory map.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn set_virtual_address_map(
        &self,
        map_size: usize,
        desc_size: usize,
        desc_version: u32,
        vmap: &MemoryDescriptor,
    ) -> Result<()> {
        (self.SetVirtualAddressMap)(map_size, desc_size, desc_version, vmap).into()
    }

    /// Determines the new virtual address that is to be used on subsequent
    /// memory accesses.
    ///
    /// # Errors
    ///
    /// - `NOT_FOUND`: The pointer pointed to by `addr` was not fouind to be a
    ///   part of the current memory map. This is normally fatal.
    /// - `INVALID_PARAMETER`: `addr` is NULL.
    /// - `INVALID_PARAMETER`: `addr` is NULL and `disp` does not have the
    ///   `OPTIONAL_PTR` bit set.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn convert_pointer(&self, disp: usize, addr: *const u8) -> Result<*const u8> {
        let mut ptr = addr;
        let status = (self.ConvertPointer)(disp, &mut ptr);

        match status {
            Status::SUCCESS => Ok(ptr),
            e => Err(e),
        }
    }

    // TODO: GetVarialbe
    // TODO: GetNextVariableName

    /// Sets the value of a variable.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: An invalid of attribute bits, name, and GUID was
    ///   suppplied, or the data size exceeds the maximum allowed.
    /// - `INVALID_PARAMETER`: `name` is an empty string.
    /// - `OUT_OF_RESOURCES`: Not enough storage is available to hold the
    ///   variable and its data.
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `WRITE_PROTECTED`: The variable is read-only.
    /// - `WRITE_PROTECTED`: The variable cannot be deleted.
    /// - `SECURITY_VIOLATION`: The variable could not be written due to
    ///   `ENHANCED_AUTHENTICATED_ACCESS` or
    ///   `TIME_BASED_AUTHENTICATED_WRITE_ACCESS` being set, but the payload
    ///   does not pass the validation check carried out by the firmware.
    /// - `NOT_FOUND`: The variable trying to be updated or deleted was not
    ///   found.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn set_variable(
        &self,
        name: &u16,
        guid: &Guid,
        attrs: VariableAttributes,
        data: &[u8],
    ) -> Result<()> {
        (self.SetVariable)(name, guid, attrs, data.len(), data.as_ptr()).into()
    }

    /// Returns the next high 32 bits of the paltform's monotonic counter.
    ///
    /// # Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: Not supported by the platform at the time the call is
    ///   made.
    pub fn get_next_high_monotonic_count(&self) -> Result<u32> {
        let mut count: u32 = 0;
        let status = (self.GetNextHighMonotonicCount)(&mut count);

        match status {
            Status::SUCCESS => Ok(count),
            err => Err(err),
        }
    }

    /// Resets the entire platform. If the platform supports
    /// `RESET_NOTIFICATION_PROTOCOL`, then prior to completing the reset of
    /// the platform, all of the pending notifications must be called.
    pub fn reset_system(&self, kind: ResetType, status: Status, reset_data: Option<&[u8]>) {
        let (size, data) = match reset_data {
            Some(dat) => (dat.len(), dat.as_ptr()),
            None => (0, core::ptr::null()),
        };

        (self.ResetSystem)(kind, status, size, data);
    }

    // TODO: UpdateCapsule
    // TODO: QueryCapsuleCapabilities

    /// Returns information about the variables.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: An invalid combination of attribute bits was
    ///   supplied.
    /// - `UNSUPPORTED`: The attribute is not supported on the platform.
    pub fn query_variable_info(&self, attr: VariableAttributes) -> Result<(u64, u64, u64)> {
        let mut storage_max_sz = 0;
        let mut storage_remaining = 0;
        let mut var_max_sz = 0;

        let status = (self.QueryVariableInfo)(
            attr,
            &mut storage_max_sz,
            &mut storage_remaining,
            &mut var_max_sz,
        );

        match status {
            Status::SUCCESS => Ok((storage_max_sz, storage_remaining, var_max_sz)),
            e => Err(e),
        }
    }
}
