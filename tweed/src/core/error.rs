use core::fmt::{Display, Formatter};

use super::value_types::ValType;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    StackExhaustion,
    CallStackExhaustion,
    UnknownFunction,
    UnknownType,
    UndefinedElement,
    InvalidConversionToInt,
    UninitializedReference,
    IndirectCallTypeMismatch,
    UnknownTable,
    MultipleMemoriesNotImplemented,
    MalformedMutability,
    FunctionNotFound,
    InvalidMagic,
    InvalidVersion,
    InvalidSectionBytecode,
    TypeMismatch,
    IncompatibleImportType,
    InvalidSignatureIndex,
    InvalidControlFlow,
    MissingValue,
    InvalidLocalIndex,
    InvalidFunctionIndex,
    InvalidMemoryIndex,
    UnknownInstance,
    UndefinedOpCode(u8),
    UndefinedContantExpression,
    UnknownLabel,
    UnknownGlobal,
    MissingGlobalImport,
    MissingFunctionImport,
    UnknownSectionCode(u8),
    UnsupportedExportKind,
    UnsupportedValueType,
    ExpectedAValueOnTheStack,
    UnexpectedValueType,
    UnknownExportCall,
    InvalidSidetableIndex,
    UndefinedInstruction(u8),
    IllegalSelectType(ValType),
    UndefinedMemory,
    UndefinedTable,
    TableAccessOutOfBounds,
    InvalidAlignment,
    MemoryAccessOutOfBounds,
    DivisionByZero,
    UnrepresentableResult,
    GlobalIsImmutable,
    IntegerOverflow,
    InstanceNameTaken,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidConversionToInt => f.write_str("Invalid conversion to int"),
            Error::StackExhaustion => f.write_str("operand stack exhausted"),
            Error::CallStackExhaustion => f.write_str("call stack exhausted"),
            Error::MalformedMutability => f.write_str("malformed mutability"),
            Error::InvalidMagic => {
                f.write_str("The magic number of the given Wasm file is invalid.")
            }
            Error::InstanceNameTaken => f.write_str("instance name is taken"),
            Error::IndirectCallTypeMismatch => f.write_str("indirect call type mismatch"),
            Error::FunctionNotFound => f.write_str("function not found"),
            Error::GlobalIsImmutable => f.write_str("global is immutable"),
            Error::DivisionByZero => f.write_str("integer divide by zero"),
            Error::IntegerOverflow => f.write_str("integer overflow"),
            Error::UnrepresentableResult => f.write_str("Result is not representable."),
            Error::InvalidVersion => f.write_str("The version in the Wasm file is invalid"),
            Error::InvalidSectionBytecode => f.write_str("The section in the Wasm file is invalid"),
            Error::TypeMismatch | Error::InvalidSignatureIndex => f.write_str("type mismatch"),
            Error::InvalidControlFlow => f.write_str("Control flow mismatch"),
            Error::MissingValue => f.write_str("Missing value on stack."),
            Error::InvalidLocalIndex => f.write_str("unknown local"),
            Error::UnknownFunction => f.write_str("unknown function"),

            Error::UndefinedElement => f.write_str("undefined element"),
            Error::InvalidFunctionIndex => f.write_str("Invalid function index"),
            Error::InvalidMemoryIndex => f.write_str("Invalid memory index"),
            Error::UndefinedContantExpression => f.write_str("constant expression required"),
            Error::MultipleMemoriesNotImplemented => {
                f.write_str("Multiple Memory proposal is not implemented")
            }
            Error::UndefinedOpCode(opcode) => {
                f.write_fmt(format_args!("Invalid opcode {opcode:#04X}"))
            }
            Error::UndefinedInstruction(instruction) => {
                f.write_fmt(format_args!("Invalid instruction {instruction:#04X}"))
            }
            Error::UnknownSectionCode(seccode) => {
                f.write_fmt(format_args!("Invalid sectioncode {seccode:#04X}"))
            }
            Error::UnknownType => f.write_str("unknown type"),

            Error::UnknownGlobal => f.write_str("unknown global"),
            Error::UnknownTable => f.write_str("unknown table"),
            Error::UninitializedReference => f.write_str("uninitialized element"),
            Error::UnsupportedExportKind => f.write_str("unsupported export kind"),
            Error::UnknownLabel => f.write_str("unknown label"),
            Error::UnsupportedValueType => f.write_str("unknown value type"),
            Error::ExpectedAValueOnTheStack => f.write_str("No value on stack"),
            Error::UnexpectedValueType => f.write_str("Unexpected value type"),
            Error::UnknownExportCall => f.write_str("Unexpected export call"),
            Error::InvalidSidetableIndex => f.write_str("Invalid sidetable entry index"),
            Error::IllegalSelectType(valtype) => {
                f.write_fmt(format_args!("Illegal select type: {valtype:?}"))
            }
            Error::MissingGlobalImport => f.write_str("missing global import"),
            Error::MissingFunctionImport => f.write_str("missing function import"),
            Error::UndefinedMemory => f.write_str("undefined memory"),
            Error::UndefinedTable => f.write_str("undefined table"),
            Error::InvalidAlignment => f.write_str("invalid alignment"),
            Error::MemoryAccessOutOfBounds => f.write_str("memory out of bounds"),
            Error::TableAccessOutOfBounds => f.write_str("out of bounds table access"),
            Error::UnknownInstance => f.write_str("unknown instance"),
            Error::IncompatibleImportType => f.write_str("incompatible import type"),
        }
    }
}
