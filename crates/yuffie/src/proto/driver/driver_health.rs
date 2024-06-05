// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Driver health protocol

use crate::prelude::*;

/// `EFI_DRIVER_HEALTH_HII_MESSAGE`
#[repr(C)]
pub struct DriverHealthHiiMessage {
    pub HiiHandle: Handle,
    pub StringId: crate::hii::StringId,
    pub MessageCode: u64,
}

/// `EFI_DRIVER_HEALTH_STATUS`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DriverHealthStatus(u32);

impl DriverHealthStatus {
    pub const HEALTHY: Self = Self(0);
    pub const REPAIR_REQUIRED: Self = Self(1);
    pub const CONFIG_REQUIRED: Self = Self(2);
    pub const FAILED: Self = Self(3);
    pub const RECONNECT_REQUIRED: Self = Self(4);
    pub const REBOOT_REQUIRED: Self = Self(5);
}

pub type DriverHealthRepairNotifyFn = extern "efiapi" fn(usize, usize) -> Status;

/// `EFI_DRIVER_HEALTH_PROTOCOL`
#[repr(C)]
pub struct DriverHealth {
    pub GetHealthStatus: extern "efiapi" fn(
        *mut Self,
        Option<Handle>,
        Option<Handle>,
        *mut DriverHealthStatus,
        *mut *mut DriverHealthHiiMessage,
        *mut Handle,
    ) -> Status,
    pub Repair: extern "efiapi" fn(
        *mut Self,
        Handle,
        Option<Handle>,
        Option<DriverHealthRepairNotifyFn>,
    ) -> Status,
}

impl DriverHealth {
    pub const GUID: Guid = guid!("2a534210-9280-41d8-ae79-cada01a2b127");
}
