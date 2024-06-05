// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Service binding protocol

use crate::prelude::*;

#[repr(C)]
pub struct ServiceBindingProtocol {
    pub CreateChild: extern "efiapi" fn(*mut Self, *mut Handle) -> Status,
    pub DestroyChild: extern "efiapi" fn(*mut Self, Handle) -> Status,
}
