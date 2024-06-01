// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Simple text protocols

use crate::prelude::*;

// 12.2. Simple Text Input Ex Protocol

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct KeyShiftState(u32);

impl KeyShiftState {
    pub const VALID: Self = Self(1 << 31);
    pub const RIGHT_SHIFT: Self = Self(1 << 0);
    pub const LEFT_SHIFT: Self = Self(1 << 1);
    pub const RIGHT_CONTROL: Self = Self(1 << 2);
    pub const LEFT_CONTROL: Self = Self(1 << 3);
    pub const RIGHT_ALT: Self = Self(1 << 4);
    pub const LEFT_ALT: Self = Self(1 << 5);
    pub const RIGHT_LOGO: Self = Self(1 << 6);
    pub const LEFT_LOGO: Self = Self(1 << 7);
    pub const MENU_KEY: Self = Self(1 << 8);
    pub const SYS_REQ: Self = Self(1 << 9);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct KeyToggleState(u8);

impl KeyToggleState {
    pub const VALID: Self = Self(0x80);
    pub const KEY_STATE_EXPOSED: Self = Self(0x40);
    pub const SCROLL_LOCK_ACTIVE: Self = Self(0x01);
    pub const NUM_LOCK_ACTIVE: Self = Self(0x02);
    pub const CAPS_LOCK_ACTIVE: Self = Self(0x04);
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct KeyState {
    pub KeyShiftState: KeyShiftState,
    pub KeyToggleState: KeyToggleState,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct KeyData {
    pub Key: InputKey,
    pub KeyState: KeyState,
}

pub type KeyNotificationFn = extern "efiapi" fn(*const KeyData) -> Status;

#[rustfmt::skip]
#[repr(C)]
pub struct SimpleTextInputEx {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub ReadKeyStrokeEx: extern "efiapi" fn(*mut Self, *mut KeyData) -> Status,
    pub WaitForKeyEx: Event,
    pub SetState: extern "efiapi" fn(*mut Self, *const KeyToggleState) -> Status,
    pub RegisterKeyNotify: extern "efiapi" fn(*mut Self, *const KeyData, KeyNotificationFn, *mut *mut Handle) -> Status,
    pub UnregisterKeyNotify: extern "efiapi" fn(*mut Self, *mut Handle) -> Status,
}

impl SimpleTextInputEx {
    pub const GUID: Guid = guid!("dd9e7534-7762-4698-8c14-f58517a625aa");
}

// 12.3 Simple Text Input Protocol

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct InputKey {
    pub ScanCode: u16,
    pub UnicodeChar: u16,
}

#[rustfmt::skip]
#[repr(C)]
pub struct SimpleTextInput {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub ReadKeyStroke: extern "efiapi" fn(*mut Self, *mut InputKey) -> Status,
    pub WaitForKey: Event,
}

impl SimpleTextInput {
    pub const GUID: Guid = guid!("387477c1-69c7-11d2-8e39-00a0c969723b");
}

// 12.4 Simple Text Output Protocol

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct SimpleTextOutputMode {
    pub MaxMode: i32,
    pub Mode: i32,
    pub Attribute: i32,
    pub CursorColumn: i32,
    pub CursorRow: i32,
    pub CursorVisible: bool,
}

#[rustfmt::skip]
#[repr(C)]
pub struct SimpleTextOutput {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub OutputString: extern "efiapi" fn(*mut Self, *const u16) -> Status,
    pub TestString: extern "efiapi" fn(*mut Self, *const u16) -> Status,
    pub QueryMode: extern "efiapi" fn(*mut Self, usize, *mut usize, *mut usize) -> Status,
    pub SetMode: extern "efiapi" fn(*mut Self, usize) -> Status,
    pub SetAttribute: extern "efiapi" fn(*mut Self, usize) -> Status,
    pub ClearScreen: extern "efiapi" fn(*mut Self) -> Status,
    pub SetCursorPosition: extern "efiapi" fn(*mut Self, usize, usize) -> Status,
    pub EnableCursor: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub Mode: *mut SimpleTextOutputMode,
}

impl SimpleTextOutput {
    pub const GUID: Guid = guid!("387477c2-69c7-11d2-8e39-00a0c969723b");
}
