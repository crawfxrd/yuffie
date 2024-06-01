// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! Internal Forms Representation
//!
//! ## References
//!
//! - UEFI Specification, version 2.10: 33.3.8 Forms Package

use core::ops;

use crate::guid;

pub type QuestionId = u16;
pub type ImageId = u16;
pub type StringId = u16;
pub type FormId = u16;
pub type VarStoreId = u16;
pub type AnimationId = u16;
pub type DefaultId = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct OpCode(u8);

impl OpCode {
    pub const FORM: Self = Self(0x01);
    pub const SUBTITLE: Self = Self(0x02);
    pub const TEXT: Self = Self(0x03);
    pub const IMAGE: Self = Self(0x04);
    pub const ONE_OF: Self = Self(0x05);
    pub const CHECKBOX: Self = Self(0x06);
    pub const NUMERIC: Self = Self(0x07);
    pub const PASSWORD: Self = Self(0x08);
    pub const ONE_OF_OPTION: Self = Self(0x09);
    pub const SUPPRESS_IF: Self = Self(0x0A);
    pub const LOCKED: Self = Self(0x0B);
    pub const ACTION: Self = Self(0x0C);
    pub const RESET_BUTTON: Self = Self(0x0D);
    pub const FORM_SET: Self = Self(0x0E);
    pub const REF: Self = Self(0x0F);
    pub const NO_SUBMIT_IF: Self = Self(0x10);
    pub const INCONSISTENT_IF: Self = Self(0x11);
    pub const EQ_ID_VAL: Self = Self(0x12);
    pub const EQ_ID_ID: Self = Self(0x13);
    pub const EQ_ID_VAL_LIST: Self = Self(0x14);
    pub const AND: Self = Self(0x15);
    pub const OR: Self = Self(0x16);
    pub const NOT: Self = Self(0x17);
    pub const RULE: Self = Self(0x18);
    pub const GRAY_OUT_IF: Self = Self(0x19);
    pub const DATE: Self = Self(0x1A);
    pub const TIME: Self = Self(0x1B);
    pub const STRING: Self = Self(0x1C);
    pub const REFRESH: Self = Self(0x1D);
    pub const DISABLE_IF: Self = Self(0x1E);
    pub const ANIMATION: Self = Self(0x1F);
    pub const TO_LOWER: Self = Self(0x20);
    pub const TO_UPPER: Self = Self(0x21);
    pub const MAP: Self = Self(0x22);
    pub const ORDERED_LIST: Self = Self(0x23);
    pub const VARSTORE: Self = Self(0x24);
    pub const VARSTORE_NAME_VALUE: Self = Self(0x25);
    pub const VARSTORE_EFI: Self = Self(0x26);
    pub const VARSTORE_DEVICE: Self = Self(0x27);
    pub const VERSION: Self = Self(0x28);
    pub const END: Self = Self(0x29);
    pub const MATCH: Self = Self(0x2A);
    pub const GET: Self = Self(0x2B);
    pub const SET: Self = Self(0x2C);
    pub const READ: Self = Self(0x2D);
    pub const WRITE: Self = Self(0x2E);
    pub const EQUAL: Self = Self(0x2F);
    pub const NOT_EQUAL: Self = Self(0x30);
    pub const GREATER_THAN: Self = Self(0x31);
    pub const GREATER_EQUAL: Self = Self(0x32);
    pub const LESS_THAN: Self = Self(0x33);
    pub const LESS_EQUAL: Self = Self(0x34);
    pub const BITWISE_AND: Self = Self(0x35);
    pub const BITWISE_OR: Self = Self(0x36);
    pub const BITWISE_NOT: Self = Self(0x37);
    pub const SHIFT_LEFT: Self = Self(0x38);
    pub const SHIFT_RIGHT: Self = Self(0x39);
    pub const ADD: Self = Self(0x3A);
    pub const SUBSTRACT: Self = Self(0x3B);
    pub const MULTIPLY: Self = Self(0x3C);
    pub const DIVIDE: Self = Self(0x3D);
    pub const MODULO: Self = Self(0x3E);
    pub const RULE_REF: Self = Self(0x3F);
    pub const QUESTION_REF1: Self = Self(0x40);
    pub const QUESTION_REF2: Self = Self(0x41);
    pub const UINT8: Self = Self(0x42);
    pub const UINT16: Self = Self(0x43);
    pub const UINT32: Self = Self(0x44);
    pub const UINT64: Self = Self(0x45);
    pub const TRUE: Self = Self(0x46);
    pub const FALSE: Self = Self(0x47);
    pub const TO_UINT: Self = Self(0x48);
    pub const TO_STRING: Self = Self(0x49);
    pub const TO_BOOLEAN: Self = Self(0x4A);
    pub const MID: Self = Self(0x4B);
    pub const FIND: Self = Self(0x4C);
    pub const TOKEN: Self = Self(0x4D);
    pub const STRING_REF1: Self = Self(0x4E);
    pub const STRING_REF2: Self = Self(0x4F);
    pub const CONDITIONAL: Self = Self(0x50);
    pub const QUESTION_REF3: Self = Self(0x51);
    pub const ZERO: Self = Self(0x52);
    pub const ONE: Self = Self(0x53);
    pub const ONES: Self = Self(0x54);
    pub const UNDEFINED: Self = Self(0x55);
    pub const LENGTH: Self = Self(0x56);
    pub const DUP: Self = Self(0x57);
    pub const THIS: Self = Self(0x58);
    pub const SPAN: Self = Self(0x59);
    pub const VALUE: Self = Self(0x5A);
    pub const DEFAULT: Self = Self(0x5B);
    pub const DEFAULTSTORE: Self = Self(0x5C);
    pub const FORM_MAP: Self = Self(0x5D);
    pub const CATENATE: Self = Self(0x5E);
    pub const GUID: Self = Self(0x5F);
    pub const SECURITY: Self = Self(0x60);
    pub const MODAL_TAG: Self = Self(0x61);
    pub const REFRESH_ID: Self = Self(0x62);
    pub const WARNING: Self = Self(0x63);
    pub const MATCH2: Self = Self(0x64);
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct OpHeader {
    /// Type of operation this header describes
    pub OpCode: OpCode,
    pub ScopeAndLength: u8,
}

impl OpHeader {
    /// The opcode begins a new scope
    pub fn scope(&self) -> u8 {
        (self.ScopeAndLength >> 7) & 0x1
    }

    /// The number of bytes in the opcode, including this header
    pub fn length(&self) -> u8 {
        self.ScopeAndLength & 0x7F
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct StatementHeader {
    pub Prompt: StringId,
    pub Help: StringId,
}

#[repr(transparent)]
pub struct QuestionFlags(u8);
impl QuestionFlags {
    pub const READ_ONLY: Self = Self(1 << 0);
    pub const CALLBACK: Self = Self(1 << 2);
    pub const RESET_REQUIRED: Self = Self(1 << 4);
    pub const REST_STYLE: Self = Self(1 << 5);
    pub const RECONNECT_REQUIRED: Self = Self(1 << 6);
    pub const OPTIONS_ONLY: Self = Self(1 << 7);
}

impl ops::BitOr for QuestionFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl From<u8> for QuestionFlags {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[repr(C)]
pub union VarStoreInfo {
    VarName: StringId,
    VarOffset: u16,
}

#[repr(C, packed)]
pub struct QuestionHeader {
    pub Header: StatementHeader,
    pub QuestionId: QuestionId,
    pub VarStoreId: VarStoreId,
    pub VarStoreInfo: VarStoreInfo,
    pub Flags: QuestionFlags,
}

#[repr(C, packed)]
pub struct Action {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub QuestionConfig: StringId,
}

#[repr(C, packed)]
pub struct Action1 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
}

#[repr(C)]
pub struct Animation {
    pub Header: OpHeader,
    pub Id: AnimationId,
}

#[repr(C)]
pub struct Add {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct And {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct BitwiseAnd {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct BitwiseNot {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct BitwiseOr {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Catenate {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Checkbox {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
}

#[repr(C)]
pub struct Conditional {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Date {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
}

// TODO
//#[repr(C)]
//pub struct Default {
//    pub Header: OpHeader,
//    pub DefaultId: u16,
//    pub Type: u8,
//    pub Value: TypeValue,
//}

#[repr(C)]
pub struct DefaultStore {
    pub Header: OpHeader,
    pub DefaultName: StringId,
    pub DefaultId: u16,
}

#[repr(C)]
pub struct DisableIf {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Divide {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Dup {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct End {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Equal {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct EqIdId {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct EqIdVal {
    pub Header: OpHeader,
    pub QuestionId: QuestionId,
    pub Value: u16,
}

// TODO
//#[repr(C)]
//pub struct EqIdValList {
//    pub Header: OpHeader,
//    pub QuestionId: QuestionId,
//    pub ListLength: u16,
//    pub ValueList: [u16; 1],
//}

#[repr(C)]
pub struct False {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Find {
    pub Header: OpHeader,
    pub Format: u8,
}

#[repr(C)]
pub struct Form {
    pub Header: OpHeader,
    pub FormId: FormId,
    pub FormTitle: StringId,
}

// TODO
//#[repr(C)]
//pub struct FormMap {
//    pub Header: OpHeader,
//    pub FormId: FormId,
//}

// TODO
//#[repr(C)]
//pub struct FormSet {
//    pub Header: OpHeader,
//    pub Guid: guid::Guid,
//    pub FormSetTitle: StringId,
//    pub Help: StringId,
//    pub Flags: u8,
//}

#[repr(C, packed)]
pub struct Get {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub VarStoreInfo: VarStoreInfo,
    pub VarStoreType: u8,
}

#[repr(C)]
pub struct GrayOutIf {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct GreaterEqual {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct GreaterThan {
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_OP_HEADER*`.
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Guid {
    pub Header: OpHeader,
    pub Guid: guid::Guid,
}

#[repr(C)]
pub struct Image {
    // NOTE: UEFI 2.10 incorrectly missing `Header`.
    pub Header: OpHeader,
    pub Id: ImageId,
}

#[repr(C)]
pub struct InconsistentIf {
    pub Header: OpHeader,
    pub Error: StringId,
}

#[repr(C)]
pub struct Length {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct LessEqual {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct LessThan {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Locked {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Map {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Match {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Mid {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Modal {
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_OP_HEADER*`.
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Modulo {
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_OP_HEADER*`.
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Multiply {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Not {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct NotEqual {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct NoSubmitIf {
    pub Header: OpHeader,
    pub Error: StringId,
}

// TODO
//#[repr(C)]
//pub struct Numeric {
//    pub Header: OpHeader,
//    pub Question: QuestionHeader,
//    pub Flags: u8,
//    pub Data: ,
//}

#[repr(C)]
pub struct One {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Ones {
    pub Header: OpHeader,
}

// TODO
//#[repr(C)]
//pub struct OneOf {
//    pub Header: OpHeader,
//    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_QUESTION_HEADER*`.
//    pub Question: QuestionHeader,
//    pub Flags: u8,
//    pub Data: ,
//}

// TODO
//#[repr(C)]
//pub struct OneOfOption {
//    pub Header: OpHeader,
//    pub Option: StringId,
//    pub Flags: u8,
//    pub Type: u8,
//    pub Value: TypeValue,
//}

#[repr(C)]
pub struct Or {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct OrderedList {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub MaxContainers: u8,
    pub Flags: u8,
}

#[repr(C, packed)]
pub struct Password {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub MinSize: u16,
    pub MaxSize: u16,
}

#[repr(C)]
pub struct QuestionRef1 {
    pub Header: OpHeader,
    pub QuestionId: QuestionId,
}

#[repr(C)]
pub struct QuestionRef2 {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct QuestionRef3 {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct QuestionRef3_2 {
    pub Header: OpHeader,
    pub DevicePath: StringId,
}

#[repr(C)]
pub struct QuestionRef3_3 {
    pub Header: OpHeader,
    pub DevicePath: StringId,
    pub Guid: guid::Guid,
}

#[repr(C)]
pub struct Read {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Ref {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
}

#[repr(C, packed)]
pub struct Ref2 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
    pub QuestionId: QuestionId,
}

#[repr(C, packed)]
pub struct Ref3 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
    pub QuestionId: QuestionId,
    pub FormSetId: guid::Guid,
}

#[repr(C, packed)]
pub struct Ref4 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
    pub QuestionId: QuestionId,
    pub FormSetId: guid::Guid,
    pub DevicePath: StringId,
}

#[repr(C, packed)]
pub struct Ref5 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
}

#[repr(C)]
pub struct Refresh {
    pub Header: OpHeader,
    pub RefreshInterval: u8,
}

#[repr(C, packed)]
pub struct RefreshId {
    pub Header: OpHeader,
    pub RefreshEventGroupId: guid::Guid,
}

#[repr(C)]
pub struct ResetButton {
    pub Header: OpHeader,
    pub Statement: StatementHeader,
    pub DefaultId: DefaultId,
}

#[repr(C)]
pub struct Rule {
    pub Header: OpHeader,
    pub RuleId: u8,
}

#[repr(C)]
pub struct RuleRef {
    pub Header: OpHeader,
    pub RuleId: u8,
}

#[repr(C, packed)]
pub struct Security {
    pub Header: OpHeader,
    pub Permissions: guid::Guid,
}

#[repr(C, packed)]
pub struct Set {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub VarStoreInfo: VarStoreInfo,
    pub VarStoreType: u8,
}

#[repr(C)]
pub struct ShiftLeft {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct ShiftRight {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Span {
    pub Header: OpHeader,
    pub Flags: u8,
}

#[repr(C, packed)]
pub struct String {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub MinSize: u8,
    pub MaxSize: u8,
    pub Flags: u8,
}

#[repr(C)]
pub struct StringRef1 {
    pub Header: OpHeader,
    pub StringId: StringId,
}

#[repr(C)]
pub struct StringRef2 {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Subtitle {
    pub Header: OpHeader,
    pub Statement: StatementHeader,
    pub Flags: u8,
}

#[repr(C)]
pub struct Subtract {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct SuprressIf {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Text {
    pub Header: OpHeader,
    pub Statement: StatementHeader,
    pub TextTwo: StringId,
}

#[repr(C)]
pub struct This {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Time {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
}

#[repr(C)]
pub struct Token {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct ToBoolean {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct ToLower {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct ToString {
    pub Header: OpHeader,
    pub Format: u8,
}

#[repr(C)]
pub struct ToUint {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct ToUpper {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct True {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct Uint8 {
    pub Header: OpHeader,
    pub Value: u8,
}

#[repr(C)]
pub struct Uint16 {
    pub Header: OpHeader,
    pub Value: u16,
}

#[repr(C, packed)]
pub struct Uint32 {
    pub Header: OpHeader,
    pub Value: u32,
}

#[repr(C, packed)]
pub struct Uint64 {
    pub Header: OpHeader,
    pub Value: u64,
}

#[repr(C)]
pub struct Undefined {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Value {
    pub Header: OpHeader,
}

// TODO
//#[repr(C)]
//pub struct VarStore {
//    pub Header: OpHeader,
//    pub Guid: guid::Guid,
//    pub VarStoreId: VarStoreId,
//    pub Size: u16,
//    //pub Name: *mut u8,
//}

#[repr(C, packed)]
pub struct VarStoreNameValue {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub Guid: guid::Guid,
}

// TODO
//#[repr(C)]
//pub struct VarStoreEfi {
//    pub Header: OpHeader,
//    pub VarStoreId: VarStoreId,
//    pub Guid: guid::Guid,
//    pub Attributes: u32,
//    pub Size: u16,
//    //pub Name: *mut u8,
//}

#[repr(C)]
pub struct VarStoreDevice {
    pub Header: OpHeader,
    pub DevicePath: StringId,
}

#[repr(C)]
pub struct Version {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Write {
    pub Header: OpHeader,
}

#[repr(C)]
pub struct Zero {
    pub Header: OpHeader,
}

#[repr(C, packed)]
pub struct WarningIf {
    pub Header: OpHeader,
    pub Warning: StringId,
    pub TimeOut: u8,
}

#[repr(C, packed)]
pub struct Match2 {
    pub Header: OpHeader,
    pub SyntaxType: guid::Guid,
}
