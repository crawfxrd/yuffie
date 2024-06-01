// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Access to commonly used data types and macros.

#[cfg(feature = "proc_macros")]
pub use yuffie_proc_macros::entry;

pub use crate::guid;
pub use crate::guid::Guid;
pub use crate::status::Result;
pub use crate::status::Status;
pub use crate::table::SystemTable;
pub use crate::Event;
pub use crate::EventType;
pub use crate::Handle;
