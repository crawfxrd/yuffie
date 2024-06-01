// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Globally Unique Identifier
//!
//! GUIDs are Microsoft's variant of [UUIDs][wiki]. The first 3 groups are
//! stored in the native endianness of the platform, as opposed to being
//! entirely in big endian like Variant 1.
//!
//! ## References
//!
//! - [UEFI Specification, Version 2.10][UEFI Spec]: Appendix A - GUID and Time
//!   Formats
//! - [Microsoft GUID structure][guiddef]
//! - [RFC 4122: A Universally Unique IDentifier (UUID) URN Namespace][rfc4122]
//!
//! [wiki]: https://en.wikipedia.org/wiki/Universally_unique_identifier
//! [guiddef]: https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid
//! [rfc4122]: https://datatracker.ietf.org/doc/html/rfc4122
//! [UEFI Spec]: https://uefi.org/sites/default/files/resources/UEFI_Spec_2_10_Aug29.pdf

use core::fmt;

/// Converts a string literal to a GUID.
#[macro_export]
macro_rules! guid {
    ($guid_str:literal) => {
        $crate::guid::Guid::parse_str($guid_str)
    };
}

/// String length of a GUID with hyphens.
const HYPHENATED_LEN: usize = 36;

/// Globally Unique Identifier
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Guid(u32, u16, u16, [u8; 8]);

impl Guid {
    /// A GUID that has all bits set to 0.
    pub const NIL: Self = guid!("00000000-0000-0000-0000-000000000000");
    /// An alias for [Guid::NIL].
    pub const NULL: Self = Self::NIL;
    /// A GUID that has all bits set to 1.
    pub const MAX: Self = guid!("ffffffff-ffff-ffff-ffff-ffffffffffff");

    /// Converts a string literal to a GUID.
    ///
    /// The string must be in the form "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".
    /// Hex digits may be either upper case or lower case.
    ///
    /// # Panics
    ///
    /// Panics if the string literal is not in the standard form.
    pub const fn parse_str(literal: &str) -> Self {
        if literal.len() != HYPHENATED_LEN {
            panic!("invalid GUID length");
        }

        let bytes = literal.as_bytes();

        // Check hyphens
        if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
            panic!("invalid GUID format");
        }

        let mut raw = [0u8; 16];

        let mut i = 0;
        let mut j = 0;
        while i < HYPHENATED_LEN {
            if i == 8 || i == 13 || i == 18 || i == 23 {
                // Already checked hyphens; skip
                i += 1;
            }

            let hi = hex_to_u8(bytes[i]);
            let lo = hex_to_u8(bytes[i + 1]);

            let b = hi << 4 | lo;
            raw[j] = b;

            i += 2;
            j += 1;
        }

        let d1 = u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]);
        let d2 = u16::from_be_bytes([raw[4], raw[5]]);
        let d3 = u16::from_be_bytes([raw[6], raw[7]]);
        let d4 = [raw[8], raw[9], raw[10], raw[11], raw[12], raw[13], raw[14], raw[15]];

        Self(d1, d2, d3, d4)
    }
}

/// Converts a hex character to its integer value.
/// Panics on invalid values.
#[doc(hidden)]
const fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => hex - b'A' + 10,
        b'a'..=b'f' => hex - b'a' + 10,
        _ => panic!("invalid hex value in GUID"),
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guid({:#010x}, {:#06x}, {:#06x}, {:?})", self.0, self.1, self.2, self.3)
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Per RFC 4122, hex digits are output as lower case characters.
        write!(f, "{:x}", &self)
    }
}

impl fmt::LowerHex for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            // Group 1
            self.0,
            // Group 2
            self.1,
            // Group 3
            self.2,
            // Group 4
            self.3[0],
            self.3[1],
            // Group 5
            self.3[2],
            self.3[3],
            self.3[4],
            self.3[5],
            self.3[6],
            self.3[7],
        )
    }
}

impl fmt::UpperHex for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            // Group 1
            self.0,
            // Group 2
            self.1,
            // Group 3
            self.2,
            // Group 4
            self.3[0],
            self.3[1],
            // Group 5
            self.3[2],
            self.3[3],
            self.3[4],
            self.3[5],
            self.3[6],
            self.3[7],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_null() {
        let guid = Guid::parse_str("00000000-0000-0000-0000-000000000000");
        assert_eq!(guid, Guid::NIL);
    }

    #[test]
    fn endianness() {
        let guid = Guid::parse_str("01020304-0506-0708-090a-0b0c0d0e0f10");
        let expected = Guid(0x01020304, 0x0506, 0x0708, [0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x10]);
        assert_eq!(guid, expected);
    }
}
