// SPDX-License-Identifier: BSD-2-Clause-Patent
// SPDX-FileCopyrightText: 2024 System76, Inc.

//! # Forms Package
//!
//! ## References
//!
//! - UEFI Specification, Version 2.10
//!   - 33.2.10: Forms Browser
//!   - 33.3.8: Forms Package

use core::ops;

use super::*;
use crate::guid;

/// `EFI_HII_TIME`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct HiiTime {
    pub Hour: u8,
    pub Minute: u8,
    pub Seconds: u8,
}

/// `EFI_HII_DATE`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct HiiDate {
    pub Year: u16,
    pub Month: u8,
    pub Day: u8,
}

/// `EFI_HII_REF`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct HiiRef {
    pub QuestionId: QuestionId,
    pub FormId: FormId,
    pub FormsetGuid: guid::Guid,
    pub DevicePath: StringId,
}

/// `EFI_IFR_TYPE_VALUE`
#[repr(C, packed)]
pub union IfrTypeValue<const N: usize = 0> {
    pub U8: u8,
    pub U16: u16,
    pub U32: u32,
    pub U64: u64,
    pub B: bool,
    pub Time: HiiTime,
    pub Date: HiiDate,
    pub String: StringId,
    pub Ref: HiiRef,
    pub Buffer: [u8; N],
}

/// IFR opcode encodings
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

/// `EFI_IFR_OP_HEADER`
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

/// `EFI_IFR_STATEMENT_HEADER`
#[derive(Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct StatementHeader {
    pub Prompt: StringId,
    pub Help: StringId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[repr(C, packed)]
pub union VarStoreInfo {
    pub VarName: StringId,
    pub VarOffset: u16,
}

/// `EFI_IFR_QUESTION_HEADER`
#[repr(C, packed)]
pub struct QuestionHeader {
    pub Header: StatementHeader,
    pub QuestionId: QuestionId,
    pub VarStoreId: VarStoreId,
    pub VarStoreInfo: VarStoreInfo,
    pub Flags: QuestionFlags,
}

/// `EFI_IFR_ACTION`
#[repr(C, packed)]
pub struct IfrAction {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub QuestionConfig: StringId,
}

/// `EFI_IFR_ACTION1`
#[repr(C, packed)]
pub struct IfrAction1 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
}

/// `EFI_IFR_ANIMATION`
#[repr(C, packed)]
pub struct IfrAnimation {
    pub Header: OpHeader,
    pub Id: AnimationId,
}

/// `EFI_IFR_ADD`
#[repr(C, packed)]
pub struct IfrAdd {
    pub Header: OpHeader,
}

/// `EFI_IFR_AND`
#[repr(C, packed)]
pub struct IfrAnd {
    pub Header: OpHeader,
}

/// `EFI_IFR_BITWISE_AND`
#[repr(C, packed)]
pub struct IfrBitwiseAnd {
    pub Header: OpHeader,
}

/// `EFI_IFR_BITWISE_NOT`
#[repr(C, packed)]
pub struct IfrBitwiseNot {
    pub Header: OpHeader,
}

/// `EFI_IFR_BITWISE_OR`
#[repr(C, packed)]
pub struct IfrBitwiseOr {
    pub Header: OpHeader,
}

/// `EFI_IFR_CATENATE`
#[repr(C, packed)]
pub struct IfrCatenate {
    pub Header: OpHeader,
}

/// `EFI_IFR_CHECKBOX`
#[repr(C, packed)]
pub struct IfrCheckbox {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
}

/// `EFI_IFR_CONDITIONAL`
#[repr(C, packed)]
pub struct IfrConditional {
    pub Header: OpHeader,
}

/// `EFI_IFR_DATE`
#[repr(C, packed)]
pub struct IfrDate {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
}

/// `EFI_IFR_DEFAULT`
#[repr(C, packed)]
pub struct IfrDefault {
    pub Header: OpHeader,
    pub DefaultId: u16,
    pub Type: u8,
    pub Value: IfrTypeValue,
}

/// `EFI_IFR_DEFAULT2`
#[repr(C, packed)]
pub struct IfrDefault2 {
    pub Header: OpHeader,
    pub DefaultId: u16,
    pub Type: u8,
}

/// `EFI_IFR_DEFAULTSTORE`
#[repr(C, packed)]
pub struct IfrDefaultStore {
    pub Header: OpHeader,
    pub DefaultName: StringId,
    pub DefaultId: u16,
}

/// `EFI_IFR_DISABLE_IF`
#[repr(C, packed)]
pub struct IfrDisableIf {
    pub Header: OpHeader,
}

/// `EFI_IFR_DIVIDE`
#[repr(C, packed)]
pub struct IfrDivide {
    pub Header: OpHeader,
}

/// `EFI_IFR_DUP`
#[repr(C, packed)]
pub struct IfrDup {
    pub Header: OpHeader,
}

/// `EFI_IFR_END`
#[repr(C, packed)]
pub struct IfrEnd {
    pub Header: OpHeader,
}

/// `EFI_IFR_EQUAL`
#[repr(C, packed)]
pub struct IfrEqual {
    pub Header: OpHeader,
}

/// `EFI_IFR_EQ_ID_ID`
#[repr(C, packed)]
pub struct IfrEqIdId {
    pub Header: OpHeader,
}

/// `EFI_IFR_EQ_ID_VAL_LIST`
#[repr(C, packed)]
pub struct IfrEqIdValList<const N: usize = 0> {
    pub Header: OpHeader,
    pub QuestionId: QuestionId,
    pub ListLength: u16,
    pub ValueList: [u16; N],
}

/// `EFI_IFR_EQ_ID_VAL`
#[repr(C, packed)]
pub struct IfrEqIdVal {
    pub Header: OpHeader,
    pub QuestionId: QuestionId,
    pub Value: u16,
}

/// `EFI_IFR_FALSE`
#[repr(C, packed)]
pub struct IfrFalse {
    pub Header: OpHeader,
}

/// `EFI_IFR_FIND`
#[repr(C, packed)]
pub struct IfrFind {
    pub Header: OpHeader,
    pub Format: u8,
}

/// `EFI_IFR_FORM`
#[repr(C, packed)]
pub struct IfrForm {
    pub Header: OpHeader,
    pub FormId: FormId,
    pub FormTitle: StringId,
}

/// `EFI_IFR_FORM_MAP_METHOD`
#[repr(C, packed)]
pub struct IfrFormMapMethod {
    pub MethodTitle: StringId,
    pub MethodIdentifier: guid::Guid,
}

/// `EFI_IFR_FORM_MAP`
#[repr(C, packed)]
pub struct IfrFormMap<const N: usize = 0> {
    pub Header: OpHeader,
    pub FormId: FormId,
    pub Methods: [IfrFormMapMethod; N],
}

/// `EFI_IFR_FORM_SET`
#[repr(C, packed)]
pub struct IfrFormSet<const N: usize = 0> {
    pub Header: OpHeader,
    pub Guid: guid::Guid,
    pub FormSetTitle: StringId,
    pub Help: StringId,
    pub Flags: u8,
    pub ClassGuid: [guid::Guid; N],
}

/// `EFI_IFR_GET`
#[repr(C, packed)]
pub struct IfrGet {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub VarStoreInfo: VarStoreInfo,
    pub VarStoreType: u8,
}

/// `EFI_IFR_GRAY_OUT_IF`
#[repr(C, packed)]
pub struct IfrGrayOutIf {
    pub Header: OpHeader,
}

/// `EFI_IFR_GREATER_EQUAL`
#[repr(C, packed)]
pub struct IfrGreaterEqual {
    pub Header: OpHeader,
}

/// `EFI_IFR_GREATER_THAN`
#[repr(C, packed)]
pub struct IfrGreaterThan {
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_OP_HEADER *`.
    pub Header: OpHeader,
}

/// `EFI_IFR_GUID`
#[repr(C, packed)]
pub struct IfrGuid {
    pub Header: OpHeader,
    pub Guid: guid::Guid,
}

/// `EFI_IFR_IMAGE`
#[repr(C, packed)]
pub struct IfrImage {
    // NOTE: UEFI 2.10 incorrectly missing `Header`.
    pub Header: OpHeader,
    pub Id: ImageId,
}

/// `EFI_IFR_INCONSISTENT_IF`
#[repr(C, packed)]
pub struct IfrInconsistentIf {
    pub Header: OpHeader,
    pub Error: StringId,
}

/// `EFI_IFR_LENGTH`
#[repr(C, packed)]
pub struct IfrLength {
    pub Header: OpHeader,
}

/// `EFI_IFR_LESS_EQUAL`
#[repr(C, packed)]
pub struct IfrLessEqual {
    pub Header: OpHeader,
}

/// `EFI_IFR_LESS_THAN`
#[repr(C, packed)]
pub struct IfrLessThan {
    pub Header: OpHeader,
}

/// `EFI_IFR_LOCKED`
#[repr(C, packed)]
pub struct IfrLocked {
    pub Header: OpHeader,
}

/// `EFI_IFR_MAP`
#[repr(C, packed)]
pub struct IfrMap {
    pub Header: OpHeader,
}

/// `EFI_IFR_MAP`
#[repr(C, packed)]
pub struct IfrMatch {
    pub Header: OpHeader,
}

/// `EFI_IFR_MID`
#[repr(C, packed)]
pub struct IfrMid {
    pub Header: OpHeader,
}

/// `EFI_IFR_MODAL_TAG`
#[repr(C, packed)]
pub struct IfrModalTag {
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_OP_HEADER *`.
    pub Header: OpHeader,
}

/// `EFI_IFR_MODULE`
#[repr(C, packed)]
pub struct IfrModulo {
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_OP_HEADER *`.
    pub Header: OpHeader,
}

/// `EFI_IFR_MULTIPLY`
#[repr(C, packed)]
pub struct IfrMultiply {
    pub Header: OpHeader,
}

/// `EFI_IFR_NOT`
#[repr(C, packed)]
pub struct IfrNot {
    pub Header: OpHeader,
}

/// `EFI_IFR_NOT_EQUAL`
#[repr(C, packed)]
pub struct IfrNotEqual {
    pub Header: OpHeader,
}

/// `EFI_IFR_NO_SUBMIT_IF`
#[repr(C, packed)]
pub struct IfrNoSubmitIf {
    pub Header: OpHeader,
    pub Error: StringId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct IfrU8Data {
    pub MinValue: u8,
    pub MaxValue: u8,
    pub Step: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct IfrU16Data {
    pub MinValue: u16,
    pub MaxValue: u16,
    pub Step: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct IfrU32Data {
    pub MinValue: u32,
    pub MaxValue: u32,
    pub Step: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct IfrU64Data {
    pub MinValue: u64,
    pub MaxValue: u64,
    pub Step: u64,
}

#[repr(C, packed)]
pub union IfrData {
    pub U8: IfrU8Data,
    pub U16: IfrU16Data,
    pub U32: IfrU32Data,
    pub U64: IfrU64Data,
}

/// `EFI_IFR_NUMERIC`
#[repr(C, packed)]
pub struct IfrNumeric {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
    pub Data: IfrData,
}

/// `EFI_IFR_ONE`
#[repr(C, packed)]
pub struct IfrOne {
    pub Header: OpHeader,
}

/// `EFI_IFR_ONES`
#[repr(C, packed)]
pub struct IfrOnes {
    pub Header: OpHeader,
}

/// `EFI_IFR_ONE_OF`
#[repr(C, packed)]
pub struct IfrOneOf {
    pub Header: OpHeader,
    // NOTE: UEFI 2.10 incorrectly uses `EFI_IFR_QUESTION_HEADER *`.
    pub Question: QuestionHeader,
    pub Flags: u8,
    pub Data: IfrData,
}

/// `EFI_IFR_ONE_OF_OPTION`
#[repr(C, packed)]
pub struct IfrOneOfOption {
    pub Header: OpHeader,
    pub Option: StringId,
    pub Flags: u8,
    pub Type: u8,
    pub Value: IfrTypeValue,
}

/// `EFI_IFR_OR`
#[repr(C, packed)]
pub struct IfrOr {
    pub Header: OpHeader,
}

/// `EFI_IFR_ORDERED_LIST`
#[repr(C, packed)]
pub struct IfrOrderedList {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub MaxContainers: u8,
    pub Flags: u8,
}

/// `EFI_IFR_PASSWORD`
#[repr(C, packed)]
pub struct IfrPassword {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub MinSize: u16,
    pub MaxSize: u16,
}

/// `EFI_IFR_QUESTION_REF1`
#[repr(C, packed)]
pub struct IfrQuestionRef1 {
    pub Header: OpHeader,
    pub QuestionId: QuestionId,
}

/// `EFI_IFR_QUESTION_REF2`
#[repr(C, packed)]
pub struct IfrQuestionRef2 {
    pub Header: OpHeader,
}

/// `EFI_IFR_QUESTION_REF3`
#[repr(C, packed)]
pub struct IfrQuestionRef3 {
    pub Header: OpHeader,
}

/// `EFI_IFR_QUESTION_REF3_2`
#[repr(C, packed)]
pub struct IfrQuestionRef3_2 {
    pub Header: OpHeader,
    pub DevicePath: StringId,
}

/// `EFI_IFR_QUESTION_REF3_3`
#[repr(C, packed)]
pub struct IfrQuestionRef3_3 {
    pub Header: OpHeader,
    pub DevicePath: StringId,
    pub Guid: guid::Guid,
}

/// `EFI_IFR_READ`
#[repr(C, packed)]
pub struct IfrRead {
    pub Header: OpHeader,
}

/// `EFI_IFR_REF`
#[repr(C, packed)]
pub struct IfrRef {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
}

/// `EFI_IFR_REF2`
#[repr(C, packed)]
pub struct IfrRef2 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
    pub QuestionId: QuestionId,
}

/// `EFI_IFR_REF3`
#[repr(C, packed)]
pub struct IfrRef3 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
    pub QuestionId: QuestionId,
    pub FormSetId: guid::Guid,
}

/// `EFI_IFR_REF4`
#[repr(C, packed)]
pub struct IfrRef4 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub FormId: FormId,
    pub QuestionId: QuestionId,
    pub FormSetId: guid::Guid,
    pub DevicePath: StringId,
}

/// `EFI_IFR_REF5`
#[repr(C, packed)]
pub struct IfrRef5 {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
}

/// `EFI_IFR_REFRESH`
#[repr(C, packed)]
#[repr(C, packed)]
pub struct IfrRefresh {
    pub Header: OpHeader,
    pub RefreshInterval: u8,
}

/// `EFI_IFR_REFRESH_ID`
#[repr(C, packed)]
pub struct IfrRefreshId {
    pub Header: OpHeader,
    pub RefreshEventGroupId: guid::Guid,
}

/// `EFI_IFR_RESET_BUTTON`
#[repr(C, packed)]
pub struct IfrResetButton {
    pub Header: OpHeader,
    pub Statement: StatementHeader,
    pub DefaultId: DefaultId,
}

/// `EFI_IFR_RULE`
#[repr(C, packed)]
pub struct IfrRule {
    pub Header: OpHeader,
    pub RuleId: u8,
}

/// `EFI_IFR_RULE_REF`
#[repr(C, packed)]
pub struct IfrRuleRef {
    pub Header: OpHeader,
    pub RuleId: u8,
}

/// `EFI_IFR_SECURITY`
#[repr(C, packed)]
pub struct IfrSecurity {
    pub Header: OpHeader,
    pub Permissions: guid::Guid,
}

/// `EFI_IFR_SET`
#[repr(C, packed)]
pub struct IfrSet {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub VarStoreInfo: VarStoreInfo,
    pub VarStoreType: u8,
}

/// `EFI_IFR_SHIFT_LEFT`
#[repr(C, packed)]
pub struct IfrShiftLeft {
    pub Header: OpHeader,
}

/// `EFI_IFR_SHIFT_RIGHT`
#[repr(C, packed)]
pub struct IfrShiftRight {
    pub Header: OpHeader,
}

/// `EFI_IFR_SPAN`
#[repr(C, packed)]
pub struct IfrSpan {
    pub Header: OpHeader,
    pub Flags: u8,
}

/// `EFI_IFR_STRING`
#[repr(C, packed)]
pub struct IfrString {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub MinSize: u8,
    pub MaxSize: u8,
    pub Flags: u8,
}

/// `EFI_IFR_STRING_REF1`
#[repr(C, packed)]
pub struct IfrStringRef1 {
    pub Header: OpHeader,
    pub StringId: StringId,
}

/// `EFI_IFR_STRING_REF2`
#[repr(C, packed)]
pub struct IfrStringRef2 {
    pub Header: OpHeader,
}

/// `EFI_IFR_SUBTITLE`
#[repr(C, packed)]
pub struct IfrSubtitle {
    pub Header: OpHeader,
    pub Statement: StatementHeader,
    pub Flags: u8,
}

/// `EFI_IFR_SUBTRACT`
#[repr(C, packed)]
pub struct IfrSubtract {
    pub Header: OpHeader,
}

/// `EFI_IFR_SUPPRESS_IF`
#[repr(C, packed)]
pub struct IfrSuprressIf {
    pub Header: OpHeader,
}

/// `EFI_IFR_TEXT`
#[repr(C, packed)]
pub struct IfrText {
    pub Header: OpHeader,
    pub Statement: StatementHeader,
    pub TextTwo: StringId,
}

/// `EFI_IFR_THIS`
#[repr(C, packed)]
pub struct IfrThis {
    pub Header: OpHeader,
}

/// `EFI_IFR_TIME`
#[repr(C, packed)]
pub struct IfrTime {
    pub Header: OpHeader,
    pub Question: QuestionHeader,
    pub Flags: u8,
}

/// `EFI_IFR_TOKEN`
#[repr(C, packed)]
pub struct IfrToken {
    pub Header: OpHeader,
}

/// `EFI_IFR_TO_BOOLEAN`
#[repr(C, packed)]
pub struct IfrToBoolean {
    pub Header: OpHeader,
}

/// `EFI_IFR_TO_LOWER`
#[repr(C, packed)]
pub struct IfrToLower {
    pub Header: OpHeader,
}

/// `EFI_IFR_TO_STRING`
#[repr(C, packed)]
pub struct IfrToString {
    pub Header: OpHeader,
    pub Format: u8,
}

/// `EFI_IFR_TO_UINT`
#[repr(C, packed)]
pub struct IfrToUint {
    pub Header: OpHeader,
}

/// `EFI_IFR_TO_UPPER`
#[repr(C, packed)]
pub struct IfrToUpper {
    pub Header: OpHeader,
}

/// `EFI_IFR_TRUE`
#[repr(C, packed)]
pub struct IfrTrue {
    pub Header: OpHeader,
}

/// `EFI_IFR_UINT8`
#[repr(C, packed)]
pub struct IfrUint8 {
    pub Header: OpHeader,
    pub Value: u8,
}

/// `EFI_IFR_UINT16`
#[repr(C, packed)]
pub struct IfrUint16 {
    pub Header: OpHeader,
    pub Value: u16,
}

/// `EFI_IFR_UINT32`
#[repr(C, packed)]
pub struct IfrUint32 {
    pub Header: OpHeader,
    pub Value: u32,
}

/// `EFI_IFR_UINT64`
#[repr(C, packed)]
pub struct IfrUint64 {
    pub Header: OpHeader,
    pub Value: u64,
}

/// `EFI_IFR_UNDEFINED`
#[repr(C, packed)]
pub struct IfrUndefined {
    pub Header: OpHeader,
}

/// `EFI_IFR_VALUE`
#[repr(C, packed)]
pub struct IfrValue {
    pub Header: OpHeader,
}

/// `EFI_IFR_VARSTORE`
#[repr(C, packed)]
pub struct IfrVarStore<const N: usize = 0> {
    pub Header: OpHeader,
    pub Guid: guid::Guid,
    pub VarStoreId: VarStoreId,
    pub Size: u16,
    pub Name: [u8; N],
}

/// `EFI_IFR_VARSTORE_NAME_VALUE`
#[repr(C, packed)]
pub struct IfrVarStoreNameValue {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub Guid: guid::Guid,
}

/// `EFI_IFR_VARSTORE_EFI`
#[repr(C, packed)]
pub struct IfrVarStoreEfi<const N: usize = 0> {
    pub Header: OpHeader,
    pub VarStoreId: VarStoreId,
    pub Guid: guid::Guid,
    pub Attributes: u32,
    pub Size: u16,
    pub Name: [u8; N],
}

/// `EFI_IFR_VARSTORE_DEVICE`
#[repr(C, packed)]
pub struct IfrVarStoreDevice {
    pub Header: OpHeader,
    pub DevicePath: StringId,
}

/// `EFI_IFR_VERSION`
#[repr(C, packed)]
pub struct IfrVersion {
    pub Header: OpHeader,
}

/// `EFI_IFR_WRITE`
#[repr(C, packed)]
pub struct IfrWrite {
    pub Header: OpHeader,
}

/// `EFI_IFR_ZERO`
#[repr(C, packed)]
pub struct IfrZero {
    pub Header: OpHeader,
}

/// `EFI_IFR_WARNING_IF`
#[repr(C, packed)]
pub struct IfrWarningIf {
    pub Header: OpHeader,
    pub Warning: StringId,
    pub TimeOut: u8,
}

/// `EFI_IFR_MATCH2`
#[repr(C, packed)]
pub struct IfrMatch2 {
    pub Header: OpHeader,
    pub SyntaxType: guid::Guid,
}
