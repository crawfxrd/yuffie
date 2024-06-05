// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Platform to driver configuration protocol

use crate::prelude::*;

/// `EFI_PLATFORM_CONFIGURATION_ACTION`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PlatformConfigurationAction(u32);

impl PlatformConfigurationAction {
    pub const NONE: Self = Self(0);
    pub const STOP_CONTROLLER: Self = Self(1);
    pub const RESTART_CONTROLLER: Self = Self(2);
    pub const RESTART_PLATFORM: Self = Self(3);
    pub const NVRAM_FAILED: Self = Self(4);
    pub const UNSUPPORTED_GUID: Self = Self(5);
}

/// `EFI_PLATFORM_TO_DRIVER_CONFIGURATION_PROTOCOL`
#[repr(C)]
pub struct PlatformToDriverConfig {
    pub Query: extern "efiapi" fn(
        *mut Self,
        Handle,
        Option<Handle>,
        *mut usize,
        *mut *mut Guid,
        *mut *mut u8,
        *mut usize,
    ) -> Status,
    pub Response: extern "efiapi" fn(
        *mut Self,
        Handle,
        Option<Handle>,
        *mut usize,
        *mut Guid,
        *mut u8,
        usize,
        PlatformConfigurationAction,
    ) -> Status,
}

impl PlatformToDriverConfig {
    pub const GUID: Guid = guid!("642cd590-8059-4c0a-a958-c5ec07d23c4b");
}
