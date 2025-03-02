// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Boot Services Table
//!
//! Wrappers to the UEFI Boot Services table.
//!
//! ## References
//!
//! - [UEFI Specification, Version 2.10][UEFI Spec]
//!   - 4.4: EFI Boot Services Table
//!   - 7: Services - Boot Services
//!
//! [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_10_Aug29.pdf

use super::Header;
use crate::Event;
use crate::Tpl;
use crate::mem::AllocateType;
use crate::mem::MemoryDescriptor;
use crate::mem::MemoryType;
use crate::mem::PhysicalAddress;
use crate::prelude::*;

pub type EventNotifyFn = extern "efiapi" fn(Event, *const u8);

#[repr(transparent)]
pub struct InterfaceType(u32);

impl InterfaceType {
    pub const NATIVE: Self = Self(0);
}

#[repr(transparent)]
pub struct LocateSearchType(u32);

impl LocateSearchType {
    pub const ALL_HANDLES: Self = Self(0);
    pub const BY_REGISTER_NOTIFY: Self = Self(1);
    pub const BY_PROTOCOL: Self = Self(2);
}

#[repr(transparent)]
pub struct TimerDelay(u32);

impl TimerDelay {
    /// The event is to be cancelled and no timer trigger is to be set.
    pub const CANCEL: Self = Self(0);
    /// The event is to be signaled periodically.
    pub const PERIODIC: Self = Self(1);
    /// The event is to be triggered in 100ms units.
    pub const RELATIVE: Self = Self(2);
}

/// The UEFI Boot Services table
#[rustfmt::skip]
#[repr(C)]
pub struct BootServices {
    pub Hdr: Header,

    // Task Priority Services
    pub RaiseTpl: extern "efiapi" fn(Tpl) -> Tpl,
    pub RestoreTpl: extern "efiapi" fn(Tpl),

    // Memory Services
    pub AllocatePages: extern "efiapi" fn(AllocateType, MemoryType, usize, *mut PhysicalAddress) -> Status,
    pub FreePages: extern "efiapi" fn(PhysicalAddress, usize) -> Status,
    pub GetMemoryMap: extern "efiapi" fn(*mut usize, *mut MemoryDescriptor, *mut usize, *mut usize, *mut u32) -> Status,
    pub AllocatePool: extern "efiapi" fn(MemoryType, usize, *mut *mut u8) -> Status,
    pub FreePool: extern "efiapi" fn(*mut u8) -> Status,

    // Event & Timer Services
    pub CreateEvent: extern "efiapi" fn(EventType, Tpl, Option<EventNotifyFn>, *const u8, *mut Event) -> Status,
    pub SetTimer: extern "efiapi" fn(Event, TimerDelay, u64) -> Status,
    pub WaitForEvent: extern "efiapi" fn(usize, *const Event, *mut usize) -> Status,
    pub SignalEvent: extern "efiapi" fn(Event) -> Status,
    pub CloseEvent: extern "efiapi" fn(Event) -> Status,
    pub CheckEvent: extern "efiapi" fn(Event) -> Status,

    // Protocol Handler Services
    pub InstallProtocolInterface: extern "efiapi" fn(*mut Handle, *const Guid, InterfaceType, *const u8) -> Status,
    pub ReinstallProtocolInterface: extern "efiapi" fn(Handle, *const Guid, *const u8, *const u8) -> Status,
    pub UninstallProtocolInterface: extern "efiapi" fn(Handle, *const Guid, *const u8) -> Status,
    pub HandleProtocol: extern "efiapi" fn(Handle, *const Guid, *mut *mut u8) -> Status,
    _Reserved: usize,
    pub RegisterProtocolNotify: extern "efiapi" fn(*const Guid, Event, *mut *mut u8) -> Status,
    pub LocateHandle: extern "efiapi" fn(LocateSearchType, *const Guid, *const u8, *mut usize, *mut Handle) -> Status,
    pub LocateDevicePath: extern "efiapi" fn(*const Guid, *mut *mut u8, *mut Handle) -> Status,
    pub InstallConfigurationTable: extern "efiapi" fn(*const Guid, *const u8) -> Status,

    // Image Services
    pub LoadImage: extern "efiapi" fn(bool, Handle,  *mut u8, *mut u8, usize, *mut Handle) -> Status,
    pub StartImage: extern "efiapi" fn(Handle, *mut usize, *mut *mut u16) -> Status,
    pub Exit: extern "efiapi" fn(Handle, Status, usize, *const u16) -> Status,
    pub UnloadImage: extern "efiapi" fn(Handle) -> Status,
    pub ExitBootServices: extern "efiapi" fn(Handle, usize) -> Status,

    // Misc Services
    pub GetNextMonotonicCount: extern "efiapi" fn(*mut u64) -> Status,
    pub Stall: extern "efiapi" fn(usize) -> Status,
    pub SetWatchdogTimer: extern "efiapi" fn(usize, u64, usize, *const u16) -> Status,

    // Driver Support Services
    pub ConnectController: extern "efiapi" fn(Handle, *const Handle, *const u8, bool) -> Status,
    pub DisconnectController: extern "efiapi" fn(Handle, *const Handle, *const Handle) -> Status,

    // Open and Close Protocol Services
    pub OpenProtocol: extern "efiapi" fn(Handle, *const Guid, *mut *mut u8, Handle, Handle, u32) -> Status,
    pub CloseProtocol: extern "efiapi" fn(Handle, *const Guid, Handle, Handle) -> Status,
    pub OpenProtocolInformation: extern "efiapi" fn(Handle, *const Guid, *mut *mut u8, *mut usize) -> Status,

    // Library Services
    pub ProtocolsPerHandle: extern "efiapi" fn(Handle, *mut *mut *mut Guid, *mut usize) -> Status,
    pub LocateHandleBuffer: extern "efiapi" fn(LocateSearchType, *const Guid, *const u8, *mut usize, *mut *mut Handle) -> Status,
    pub LocateProtocol: extern "efiapi" fn(*const Guid, *const u8, *mut *mut u8) -> Status,
    // XXX: Variadic arguments: https://github.com/rust-lang/rust/issues/100189
    InstallMultipleProtocolInterfaces: extern "efiapi" fn(*mut Handle) -> Status,
    // XXX: Variadic arguments: https://github.com/rust-lang/rust/issues/100189
    UninstallMultipleProtocolInterfaces: extern "efiapi" fn(Handle) -> Status,

    // 32-bit CRC Services
    pub CalculateCrc32: extern "efiapi" fn(*const u8, usize, *mut u32) -> Status,

    // Misc Services
    pub CopyMem: extern "efiapi" fn(*mut u8, *mut u8, usize),
    pub SetMem: extern "efiapi" fn(*mut u8, usize, u8),
    pub CreateEventEx: extern "efiapi" fn(EventType, Tpl, Option<EventNotifyFn>, *const u8, *const Guid, *mut Event) -> Status,
}

impl BootServices {
    pub fn header(&self) -> &Header {
        &self.Hdr
    }

    /// Raises a task's priority level and returns its previous level.
    pub fn raise_tpl(&self, tpl: Tpl) -> Tpl {
        (self.RaiseTpl)(tpl)
    }

    /// Restores a task's priority level to its previous value.
    pub fn restore_tpl(&self, tpl: Tpl) {
        (self.RestoreTpl)(tpl);
    }

    /// Allocates memory pages from the system.
    ///
    /// # Errors
    ///
    /// - `OUT_OF_RESOURCES`: The pages could not be allocated.
    /// - `INVALID_PARAMETER`: `mem_type` is `PERSISTENT` or `UNACCEPTED`.
    pub fn allocate_pages(
        &self,
        alloc_type: AllocateType,
        mem_type: MemoryType,
        pages: usize,
    ) -> Result<PhysicalAddress> {
        let mut memory = PhysicalAddress::from(0);
        let status = (self.AllocatePages)(alloc_type, mem_type, pages, &mut memory);

        match status {
            Status::SUCCESS => Ok(memory),
            e => Err(e),
        }
    }

    /// Frees memory pages.
    ///
    /// # Errors
    ///
    /// - `NOT_FOUND`: The requests memory pages were not allocated with
    ///   `AllocatePages()`.
    /// - `INVALID_PARAMETER`: `memory` is not a page-aligned address.
    /// - `INVALID_PARAMETER`: `pages` is invalid.
    pub fn free_pages(&self, memory: PhysicalAddress, pages: usize) -> Result<()> {
        (self.FreePages)(memory, pages).into()
    }

    // TODO: GetMemoryMap

    /// Allocates pool memory.
    ///
    /// # Errors
    ///
    /// - `OUT_OF_RESOURCES`: The pool requested could not be allocated.
    /// - `INVALID_PARAMETER`: `pool_type` is `PERSISTENT`.
    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut u8> {
        let mut buffer = core::ptr::null_mut();
        match (self.AllocatePool)(pool_type, size, &mut buffer) {
            Status::SUCCESS => Ok(buffer),
            e => Err(e),
        }
    }

    /// Returns pool memory to the system.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: `buffer` is invalid.
    pub fn free_pool(&self, buffer: *mut u8) -> Result<()> {
        (self.FreePool)(buffer).into()
    }

    /// Creates an event.
    ///
    /// # Errors
    ///
    /// - `OUT_OF_RESOURCES`: The event could not be allocated.
    /// - `INVALID_PARAMETER`: One of the parameters has an invalid value.
    /// - `INVALID_PARAMETER`: `event_type` has an unsupported bit set.
    /// - `INVALID_PARAMETER`: `event_type` has both `NOTIFY_SIGNAL` and
    ///   `NOTIFY_WAIT` set.
    /// - `INVALID_PARAMETER`: `event_type` has either `NOTIFY_SIGNAL` or
    ///   `NOTIFY_WAIT` set and `notify` is `None`.
    /// - `INVALID_PARAMETER`: `event_type` has either `NOTIFY_SIGNAL` or
    ///   `NOTIFY_WAIT` set and `tpl` is not a supported level.
    pub fn create_event(
        &self,
        event_type: EventType,
        tpl: Tpl,
        notify: Option<EventNotifyFn>,
        context: Option<&[u8]>,
    ) -> Result<Event> {
        let mut event = unsafe { Event::uninit() };
        let ctx = match context {
            Some(c) => c.as_ptr(),
            None => core::ptr::null(),
        };

        let status = (self.CreateEvent)(event_type, tpl, notify, ctx, &mut event);
        match status {
            Status::SUCCESS => Ok(event),
            e => Err(e),
        }
    }

    /// Sets the type of timer and the trigger time for a timer event.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: `event` or `kind` is not valid.
    pub fn set_timer(&self, event: Event, kind: TimerDelay, time: u64) -> Result<()> {
        (self.SetTimer)(event, kind, time).into()
    }

    /// Stops execution until an event is signaled.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: The event indicated is of type `NOTIFY_SIGNAL`.
    /// - `UNSUPPORTED`: The current TPL is not `APPLICATION`.
    pub fn wait_for_event(&self, events: &[Event]) -> Result<usize> {
        let mut index = 0;
        let status = (self.WaitForEvent)(events.len(), events.as_ptr(), &mut index);

        match status {
            Status::SUCCESS => Ok(index),
            e => Err(e),
        }
    }

    /// Signals an event.
    ///
    /// # Errors
    ///
    /// - None documented
    pub fn signal_event(&self, event: Event) -> Result<()> {
        (self.SignalEvent)(event).into()
    }

    /// Closes an event.
    ///
    /// # Errors
    ///
    /// - None documented
    pub fn close_event(&self, event: Event) -> Result<()> {
        (self.CloseEvent)(event).into()
    }

    /// Checks whether an event is in the signaled state.
    ///
    /// # Errors
    ///
    /// - `NOT_READY`: The event is not in the signaled state.
    /// - `INVALID_PARAMETER`: `event` is of type `NOTIFY_SIGNAL`.
    pub fn check_event(&self, event: Event) -> Result<()> {
        (self.CheckEvent)(event).into()
    }

    /// Installs a protocol interface on a device handle.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: `protocol` is already installed on `handle`.
    /// - `OUT_OF_RESOURCES`: Space for a new handle could not be allocated.
    pub fn install_protocol_interface(
        &self,
        protocol: &Guid,
        interface: *const u8,
    ) -> Result<Handle> {
        let mut handle = unsafe { Handle::uninit() };
        let status = (self.InstallProtocolInterface)(
            &mut handle,
            protocol,
            InterfaceType::NATIVE,
            interface,
        );

        match status {
            Status::SUCCESS => Ok(handle),
            e => Err(e),
        }
    }

    // TODO: ReinstallProtocolInterface

    /// Removes a protocol interface from a device handle.
    ///
    /// # Errors
    ///
    /// - `NOT_FOUND`: The interface was not found.
    /// - `ACCESS_DENIED`: The interface was not removed because the interface
    ///   is still being used by a driver.
    pub fn uninstall_protocol_interface(
        &self,
        handle: Handle,
        protocol: &Guid,
        interface: *const u8,
    ) -> Result<()> {
        (self.UninstallProtocolInterface)(handle, protocol, interface).into()
    }

    // TODO; HandleProtocol
    // TODO; RegisterProtocolNotify
    // TODO: LocateHandle
    // TODO: LocateDevicePath

    /// Adds, updates, or removes a configuration table entry from the system
    /// table.
    ///
    /// # Errors
    ///
    /// - `NOT_FOUND`: An attempt was made to delete a non-existent entry.
    /// - `OUT_OF_RESOURCES`: There is not enough memory available to complete
    ///   the operation.
    pub fn install_configuration_table(&self, guid: &Guid, table: Option<&[u8]>) -> Result<()> {
        let table = match table {
            Some(t) => t.as_ptr(),
            None => core::ptr::null(),
        };

        (self.InstallConfigurationTable)(guid, table).into()
    }

    // TODO: LoadImage
    // TODO: StartImage

    /// Terminates a loaded EFI image and returns control to boot services.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: The image specified has been loaded and started,
    ///   but the image is not the currently executing image.
    pub fn exit(&self, image: Handle, exit_status: Status, data: Option<&[u16]>) -> Result<()> {
        let (data, data_size) = match data {
            Some(d) => (d.as_ptr(), d.len()),
            None => (core::ptr::null(), 0),
        };
        (self.Exit)(image, exit_status, data_size, data).into()
    }

    /// Unloads an image.
    ///
    /// # Errors
    ///
    /// - The exit code from the image's unload function
    /// - `UNSUPPORTED`: The image has been started, and does not support
    ///   unload.
    /// - `INVALID_PARAMETER`: `image` is not a valid image handle.
    pub fn unload_image(&self, image: Handle) -> Result<()> {
        (self.UnloadImage)(image).into()
    }

    /// Terminates all boot services.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: `key` is incorrect
    pub fn exit_boot_services(&self, image: Handle, key: usize) -> Result<()> {
        // TODO: Update SystemTable to invalidate fields
        (self.ExitBootServices)(image, key).into()
    }

    /// Returns a monotonically increasing count for the platform.
    ///
    /// # Errors
    ///
    /// - `DEVICE_ERROR`: The device is not functioning properly
    pub fn get_next_monotonic_count(&self) -> Result<u64> {
        let mut count: u64 = 0;
        let status = (self.GetNextMonotonicCount)(&mut count);

        match status {
            Status::SUCCESS => Ok(count),
            e => Err(e),
        }
    }

    /// Induces a fine-grained stall.
    ///
    /// # Errors
    ///
    /// - None documented
    pub fn stall(&self, microseconds: usize) -> Result<()> {
        (self.Stall)(microseconds).into()
    }

    /// Resets and sets a watchdog timer used during boot services time.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: The supplied `code` is invalid.
    /// - `UNSUPPORTED`: The system does not have a watchdog timer.
    /// - `DEVICE_ERROR`: The watchdog timer could not be programmed due to a
    ///   hardware error.
    pub fn set_watchdog_timer(
        &self,
        timeout: usize,
        code: u64,
        data: Option<&[u16]>,
    ) -> Result<()> {
        let (data, size) = match data {
            Some(d) => (d.as_ptr(), d.len()),
            None => (core::ptr::null(), 0),
        };

        (self.SetWatchdogTimer)(timeout, code, size, data).into()
    }

    // TODO: ConnectController
    // TODO: DisconnectController
    // TODO: OpenProtocol
    // TODO: CloseProtocol
    // TODO: OpenProtocolInformation
    // TODO: ProtocolsPerHandle
    // TODO: LocateHandleBuffer
    // TODO: LocateProtocol
    // TODO: InstallMultipleProtocolInterfaces
    // TODO: UninstallMultipleProtocolInterfaces

    /// Computes and returns a 32-bit CRC for a data buffer.
    ///
    /// # Errors
    ///
    /// - `INVALID_PARAMETER`: `data`'s size is 0.
    pub fn crc32(&self, data: &[u8]) -> Result<u32> {
        let mut crc32 = 0;
        let status = (self.CalculateCrc32)(data.as_ptr(), data.len(), &mut crc32);

        match status {
            Status::SUCCESS => Ok(crc32),
            _ => Err(status),
        }
    }

    /// Copies the content of one buffer to another.
    pub fn copy_mem(&self, dest: &mut [u8], src: &mut [u8], size: usize) {
        (self.CopyMem)(dest.as_mut_ptr(), src.as_mut_ptr(), size);
    }

    /// Fills a buffer with a specified value.
    pub fn set_mem(&self, buffer: &mut [u8], size: usize, value: u8) {
        (self.SetMem)(buffer.as_mut_ptr(), size, value);
    }

    /// Creates an event in a group.
    ///
    /// # Errors
    ///
    /// - `OUT_OF_RESOURCES`: The event could not be allocated.
    /// - `INVALID_PARAMETER`: `event_type` has both `NOTIFY_SIGNAL` and
    ///   `NOTIFY_WAIT` set.
    /// - `INVALID_PARAMETER`: `event_type` has either `NOTIFY_SIGNAL` or
    ///   `NOTIFY_WAIT` set and `notify` is `None`.
    /// - `INVALID_PARAMETER`: `event_type` has either `NOTIFY_SIGNAL` or
    ///   `NOTIFY_WAIT` set and `tpl` is not a supported level.
    pub fn create_event_ex(
        &self,
        event_type: EventType,
        tpl: Tpl,
        notify: Option<EventNotifyFn>,
        context: Option<&[u8]>,
        group: Option<&Guid>,
    ) -> Result<Event> {
        let mut event = unsafe { Event::uninit() };
        let guid = match group {
            Some(g) => g,
            None => core::ptr::null(),
        };
        let ctx = match context {
            Some(c) => c.as_ptr(),
            None => core::ptr::null(),
        };

        let status = (self.CreateEventEx)(event_type, tpl, notify, ctx, guid, &mut event);

        match status {
            Status::SUCCESS => Ok(event),
            e => Err(e),
        }
    }
}
