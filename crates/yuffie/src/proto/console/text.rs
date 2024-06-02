// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Simple text protocols

use core::ptr;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

/// `EFI_KEY_TOGGLE_STATE`
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct KeyToggleState(u8);

impl KeyToggleState {
    pub const VALID: Self = Self(0x80);
    pub const KEY_STATE_EXPOSED: Self = Self(0x40);
    pub const SCROLL_LOCK_ACTIVE: Self = Self(0x01);
    pub const NUM_LOCK_ACTIVE: Self = Self(0x02);
    pub const CAPS_LOCK_ACTIVE: Self = Self(0x04);
}

/// `EFI_KEY_STATE`
#[derive(Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct KeyState {
    pub KeyShiftState: KeyShiftState,
    pub KeyToggleState: KeyToggleState,
}

/// `EFI_KEY_DATA`
#[derive(Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct KeyData {
    pub Key: InputKey,
    pub KeyState: KeyState,
}

/// `EFI_KEY_NOTIFY_FUNCTION`
pub type KeyNotificationFn = extern "efiapi" fn(*const KeyData) -> Status;

/// `EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`
#[rustfmt::skip]
#[repr(C)]
pub struct SimpleTextInputEx {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub ReadKeyStrokeEx: extern "efiapi" fn(*mut Self, *mut KeyData) -> Status,
    pub WaitForKeyEx: Event,
    pub SetState: extern "efiapi" fn(*mut Self, *const KeyToggleState) -> Status,
    pub RegisterKeyNotify: extern "efiapi" fn(*mut Self, *const KeyData, KeyNotificationFn, *mut *mut u8) -> Status,
    pub UnregisterKeyNotify: extern "efiapi" fn(*mut Self, *const u8) -> Status,
}

impl SimpleTextInputEx {
    pub const GUID: Guid = guid!("dd9e7534-7762-4698-8c14-f58517a625aa");

    //// Resets the input device hardware.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    pub fn reset(&mut self, verify: bool) -> Result<()> {
        (self.Reset)(self, verify).into()
    }

    /// Reads the next keystroke from the input device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `NOT_READY`: There was no keystroke available.
    /// - `UNSUPPORTED`: The device does not support the ability to read
    ///   keystroke data.
    pub fn read_key(&mut self) -> Result<KeyData> {
        let mut key = KeyData::default();
        let status = (self.ReadKeyStrokeEx)(self, &mut key);

        match status {
            Status::SUCCESS => Ok(key),
            err => Err(err),
        }
    }

    /// Sets the state for the input device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The device does not support the ability to have its
    ///   state set or the requested state change was not supported.
    pub fn set_state(&mut self, state: &KeyToggleState) -> Result<()> {
        (self.SetState)(self, state).into()
    }

    /// Registers a notification function for a particular keystroke for the
    /// input device.
    ///
    /// ## Errors
    ///
    /// - `OUT_OF_RESOURCES`: Unable to allocate necessart data structures.
    pub fn register_key_notify(
        &mut self,
        key: &KeyData,
        cb: KeyNotificationFn,
    ) -> Result<*const u8> {
        let mut handle = ptr::null_mut();
        let status = (self.RegisterKeyNotify)(self, key, cb, &mut handle);

        match status {
            Status::SUCCESS => Ok(handle),
            err => Err(err),
        }
    }

    /// Removes the notification that was previously registered.
    ///
    /// ## Errors
    ///
    /// - `INVALID_PARAMETER`: The handle is invalid
    pub fn unregister_key_notify(&mut self, handle: &u8) -> Result<()> {
        (self.UnregisterKeyNotify)(self, handle).into()
    }
}

/// `EFI_INPUT_KEY`
#[derive(Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct InputKey {
    pub ScanCode: u16,
    pub UnicodeChar: u16,
}

/// `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`
#[rustfmt::skip]
#[repr(C)]
pub struct SimpleTextInput {
    pub Reset: extern "efiapi" fn(*mut Self, bool) -> Status,
    pub ReadKeyStroke: extern "efiapi" fn(*mut Self, *mut InputKey) -> Status,
    pub WaitForKey: Event,
}

impl SimpleTextInput {
    pub const GUID: Guid = guid!("387477c1-69c7-11d2-8e39-00a0c969723b");

    //// Resets the input device hardware.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    pub fn reset(&mut self, verify: bool) -> Result<()> {
        (self.Reset)(self, verify).into()
    }

    /// Reads the next keystroke from the input device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `NOT_READY`: There was no keystroke available.
    /// - `UNSUPPORTED`: The device does not support the ability to read
    ///   keystroke data.
    pub fn read_key(&mut self) -> Result<InputKey> {
        let mut key = InputKey::default();
        let status = (self.ReadKeyStroke)(self, &mut key);

        match status {
            Status::SUCCESS => Ok(key),
            err => Err(err),
        }
    }
}

/// `EFI_SIMPLE_TEXT_OUTPUT_MODE`
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

/// `EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`
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

    /// Resets the text output device hardware.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    pub fn reset(&mut self, verify: bool) -> Result<()> {
        (self.Reset)(self, verify).into()
    }

    /// Writes a string to the output device.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The output device's mode is not currently in a defined
    ///   text mode.
    pub fn output_string(&mut self, string: *const u16) -> Result<()> {
        (self.OutputString)(self, string).into()
    }

    /// Verifies that all characters in a string can be output to the target
    /// device.
    ///
    /// ## Errors
    ///
    /// - `UNSUPPORTED`: Some of the characters in the string cannot be rendered
    ///   by one or more of the output devices mapped.
    pub fn test_string(&mut self, string: *const u16) -> Result<()> {
        (self.TestString)(self, string).into()
    }

    /// Returns information for an available text mode that the ouput supports.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The mode number was not valid.
    pub fn query_mode(&mut self, index: usize) -> Result<(usize, usize)> {
        let (mut cols, mut rows) = (0, 0);
        let status = (self.QueryMode)(self, index, &mut cols, &mut rows);

        match status {
            Status::SUCCESS => Ok((cols, rows)),
            err => Err(err),
        }
    }

    /// Sets the output device to a specified mode.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The mode number was not valid.
    pub fn set_mode(&mut self, index: usize) -> Result<()> {
        (self.SetMode)(self, index).into()
    }

    /// Sets the background and foreground colors for the output string.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error
    pub fn set_attribute(&mut self, attr: usize) -> Result<()> {
        (self.SetAttribute)(self, attr).into()
    }

    /// Clears the output device display to the currently selected background
    /// color.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The output device is not in a valid text mode.
    pub fn clear_screen(&mut self) -> Result<()> {
        (self.ClearScreen)(self).into()
    }

    /// Sets the current coordinates of the cursor position.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The output device is not in a valid text mode, or the
    ///   cursor position is invalid for the current mode.
    pub fn set_cursor_position(&mut self, col: usize, row: usize) -> Result<()> {
        (self.SetCursorPosition)(self, col, row).into()
    }

    /// Makes the cursor visible or invisible.
    ///
    /// ## Errors
    ///
    /// - `DEVICE_ERROR`: Hardware error.
    /// - `UNSUPPORTED`: The output device does not support visibility control
    ///   of the cursor.
    pub fn enable_cursor(&mut self, visible: bool) -> Result<()> {
        (self.EnableCursor)(self, visible).into()
    }
}
