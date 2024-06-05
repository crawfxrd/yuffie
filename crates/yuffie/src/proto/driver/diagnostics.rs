// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Driver diagnostics protocol

use crate::prelude::*;

/// `EFI_DRIVER_DIAGNOSTIC_TYPE`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DriverDiagnosticType(u32);

impl DriverDiagnosticType {
    pub const STANDARD: Self = Self(0);
    pub const EXTENDED: Self = Self(1);
    pub const MANUFACTURING: Self = Self(2);
    pub const CANCEL: Self = Self(3);
}

/// `EFI_DRIVER_DIAGNOSTICS2_PROTOCOL`
#[repr(C)]
pub struct DriverDiagnostics2 {
    pub RunDiagnostics: extern "efiapi" fn(
        *mut Self,
        Handle,
        Option<Handle>,
        DriverDiagnosticType,
        *mut u8,
        *mut *mut Guid,
        *mut usize,
        *mut *mut u16,
    ) -> Status,
    pub SupportedLanguages: *mut u8,
}

impl DriverDiagnostics2 {
    pub const GUID: Guid = guid!("4d330321-025f-4aac-90d8-5ed900173b63");
}
