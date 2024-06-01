// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! System Table

use super::Header;
use crate::prelude::*;
use crate::proto::console::text::SimpleTextInput;
use crate::proto::console::text::SimpleTextOutput;
use crate::table::BootServices;
use crate::table::RuntimeServices;

// TODO: Handle `ExitBootServices` making most fields invalid.

/// The UEFI System table
#[repr(C)]
pub struct SystemTable {
    /// The system table header.
    pub Hdr: Header,
    /// A pointer to a null-terminated string string that identifies the
    /// vendor that produced the system firmware for the platform.
    pub FirmwareVendor: *const u16,
    /// A vendor-specific value that identifies the revision of the system
    /// firmware for the platform.
    pub FirmwareRevision: u32,
    /// The handle for the active console input device.
    pub ConsoleInHandle: Handle,
    /// A pointer to the protocol interface associated with `ConsoleInHandle`.
    pub ConIn: *mut SimpleTextInput,
    /// The handle to the active console output device.
    pub ConsoleOutHandle: Handle,
    /// A pointer to the protocol interface associated with `ConsoleOutHandle`.
    pub ConOut: *mut SimpleTextOutput,
    /// The handle to the active standard error console device.
    pub StandardErrorHandle: Handle,
    /// A pointer to the protocol interface associated with
    /// `StandardErrorHandle`.
    pub StdErr: *mut SimpleTextOutput,
    /// A pointer to the Runtime Services table.
    pub RuntimeServices: *mut RuntimeServices,
    /// A pointer to the Boot Services table.
    pub BootServices: *mut BootServices,
    /// The number of system configuration tables in `ConfigurationTable`.
    pub NumberOfTableEntries: usize,
    /// A pointer to the system configuration tables.
    pub ConfigurationTable: *mut usize,
}

impl SystemTable {
    pub fn header(&self) -> &Header {
        &self.Hdr
    }

    pub fn revision(&self) -> u32 {
        self.FirmwareRevision
    }

    pub fn boot_services(&mut self) -> &mut BootServices {
        unsafe { &mut *self.BootServices }
    }

    pub fn runtime_services(&mut self) -> &mut RuntimeServices {
        unsafe { &mut *self.RuntimeServices }
    }

    pub fn stdin(&mut self) -> &mut SimpleTextInput {
        unsafe { &mut *self.ConIn }
    }

    pub fn stdout(&mut self) -> &mut SimpleTextOutput {
        unsafe { &mut *self.ConOut }
    }

    pub fn stderr(&mut self) -> &mut SimpleTextOutput {
        unsafe { &mut *self.StdErr }
    }
}
