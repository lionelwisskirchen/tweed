use core::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, WasmError>;

/// A unified error type for the WebAssembly Runtime.
///
/// `WasmError` represents all errors that can occur while interpreting
/// WebAssembly modules.
///
/// # Variants
///
/// - `ValidationError`
///   Errors that occur during the *validation* phase, such as malformed
///   bytecode or violations of the WebAssembly specification.
///
/// - `ExecutionError`
///   Errors that occur during the *execution* phase, after successful
///   validation, such as missing instances or runtime failures.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WasmError {
    Validation(ValidationError),
    Execution(ExecutionError),
}
impl From<ValidationError> for WasmError {
    fn from(e: ValidationError) -> Self {
        WasmError::Validation(e)
    }
}

impl From<ExecutionError> for WasmError {
    fn from(e: ExecutionError) -> Self {
        WasmError::Execution(e)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExecutionError {
    InstanceNameTaken,
    UnknownReader,
    UnknownInstance,
    UnknownImport,
    DivisionByZero,
    IntegerOverflow,
    MissingFunctionImport,
    IncompatibleImportType,
    MissingGlobalImport,
    TypeMismatch,
    InvalidFunctionIndex,
    InvalidMemoryIndex,
    UndefinedContantExpression,
    TrapUnreachable,
    StackExhaustion,
    CallStackExhaustion,
    ExpectedAValueOnTheStack,
    UnexpectedValueType,
    UnknownExportCall,
    MemoryAccessOutOfBounds,
    TableAccessOutOfBounds,
    InvalidLocalIndex,
    UnknownGlobal,
    UnknownTable,
    UndefinedElement,
    UninitializedReference,
    InvalidSidetableIndex,
    IndirectCallTypeMismatch,
    InvalidConversionToInt,
    InvalidSignatureIndex,
    FunctionNotFound,
    MissingTableImport,
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InstanceNameTaken => f.write_str("instance name taken"),
            Self::UnknownReader => f.write_str("no reader found for instance"),
            Self::UnknownInstance => f.write_str("unknown instance"),
            Self::UnknownImport => f.write_str("unknown import"),
            Self::MissingFunctionImport => f.write_str("missing function import"),
            Self::IncompatibleImportType => f.write_str("incompatible import type"),
            Self::MissingGlobalImport => f.write_str("missing global import"),
            Self::TypeMismatch => f.write_str("type mismatch"),
            Self::InvalidFunctionIndex => f.write_str("invalid function index"),
            Self::InvalidMemoryIndex => f.write_str("invalid memory index"),
            Self::UndefinedContantExpression => f.write_str("constant expression required"),
            Self::TrapUnreachable => f.write_str("unreachable"),
            Self::StackExhaustion => f.write_str("operand stack exhausted"),
            Self::CallStackExhaustion => f.write_str("call stack exhausted"),
            Self::ExpectedAValueOnTheStack => f.write_str("No value on stack"),
            Self::UnexpectedValueType => f.write_str("unexpected value type"),
            Self::UnknownExportCall => f.write_str("unexpected export call"),
            Self::MemoryAccessOutOfBounds => f.write_str("out of bounds memory access"),
            Self::TableAccessOutOfBounds => f.write_str("out of bounds table access"),
            Self::InvalidLocalIndex => f.write_str("unknown local"),
            Self::UnknownGlobal => f.write_str("unknown global"),
            Self::UnknownTable => f.write_str("unknown table"),
            Self::DivisionByZero => f.write_str("integer divide by zero"),
            Self::IntegerOverflow => f.write_str("integer overflow"),
            Self::UndefinedElement => f.write_str("undefined element"),
            Self::UninitializedReference => f.write_str("uninitialized element"),
            Self::InvalidSidetableIndex => f.write_str("invalid sidetable entry index"),
            Self::IndirectCallTypeMismatch => f.write_str("indirect call type mismatch"),
            Self::InvalidConversionToInt => f.write_str("invalid conversion to integer"),
            Self::InvalidSignatureIndex => f.write_str("invalid signature index"),
            Self::FunctionNotFound => f.write_str("function not found"),
            Self::MissingTableImport => f.write_str("missing table import"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValidationError {
    ReadingOutOfBounds,
    IntegerTooLarge,
    InvalidIntegerRepresentation,
    InvalidStartFunctionSignature,
    NoDataSegmentFound(Option<usize>),
    UnknownMemory(usize),
    UnknownFunction,
    UnknownType,
    UnknownTable,
    MultipleMemoriesNotImplemented,
    MalformedMutability,
    InvalidMagic,
    InvalidVersion,
    TypeMismatch,
    InvalidSignatureIndex,
    InvalidControlFlow,
    InvalidLocalIndex,
    InvalidFunctionIndex,
    UndefinedOpCode(u8),
    UndefinedContantExpression,
    UnknownLabel,
    UnknownGlobal,
    UnknownSectionCode(u8),
    UnsupportedExportKind,
    UnsupportedValueType,
    IllegalSelectType,
    UndefinedMemory,
    UndefinedTable,
    InvalidAlignment,
    InvalidResultArity,
    GlobalIsImmutable,
    MalformedMemOpFlags,
    SizeMinimumMustNotBeGreaterThanMaximum,
    UnsupportedImportKind,
    InvalidStringRepresentation,
    UnequalChunkComparison,
    InvalidElementKind,
    MissingValue
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidStringRepresentation => f.write_str("invalid string representation"),
            Self::UnequalChunkComparison => f.write_str("unequal chunk comparison"),
            Self::ReadingOutOfBounds => f.write_str("reading out of bounds"),
            Self::MalformedMemOpFlags => f.write_str("malformed memop flags"),
            Self::InvalidStartFunctionSignature => f.write_str("start function"),
            Self::InvalidResultArity => f.write_str("invalid result arity"),
            Self::IntegerTooLarge => f.write_str("integer too large"),
            Self::NoDataSegmentFound(idx) => {
                if let Some(i) = idx {
                    f.write_fmt(format_args!("unknown data segment {i}"))
                } else {
                    f.write_str("unknown data segment")
                }
            }
            Self::MalformedMutability => f.write_str("malformed mutability"),
            Self::InvalidMagic => f.write_str("invalid magic number"),
            Self::GlobalIsImmutable => f.write_str("global is immutable"),
            Self::InvalidIntegerRepresentation => f.write_str("integer representation too long"),
            Self::InvalidVersion => f.write_str("invalid wasm version"),
            Self::TypeMismatch | Self::InvalidSignatureIndex => f.write_str("type mismatch"),
            Self::InvalidControlFlow => f.write_str("invalid control flow"),
            Self::InvalidLocalIndex => f.write_str("unknown local"),
            Self::UnknownFunction => f.write_str("unknown function"),

            Self::InvalidFunctionIndex => f.write_str("invalid function index"),
            Self::UndefinedContantExpression => f.write_str("constant expression required"),
            Self::MultipleMemoriesNotImplemented => {
                f.write_str("multiple Memory proposal is not implemented")
            }
            Self::UndefinedOpCode(opcode) => {
                f.write_fmt(format_args!("invalid opcode {opcode:#04X}"))
            }
            Self::UnknownSectionCode(seccode) => {
                f.write_fmt(format_args!("unknown sectioncode {seccode:#04X}"))
            }
            Self::UnknownType => f.write_str("unknown type"),

            Self::UnknownGlobal => f.write_str("unknown global"),
            Self::UnknownTable => f.write_str("unknown table"),
            Self::UnsupportedExportKind => f.write_str("unsupported export kind"),
            Self::UnknownLabel => f.write_str("unknown label"),
            Self::UnsupportedValueType => f.write_str("unknown value type"),
            Self::IllegalSelectType => f.write_str("type mismatch"),

            Self::UndefinedMemory => f.write_str("undefined memory"),
            Self::UndefinedTable => f.write_str("undefined table"),
            Self::UnknownMemory(idx) => f.write_fmt(format_args!("unknown memory {idx}")),
            Self::InvalidAlignment => f.write_str("alignment must not be larger than natural"),

            Self::InvalidElementKind => f.write_str("invalid element kind"),
            Self::SizeMinimumMustNotBeGreaterThanMaximum => {
                f.write_str("size minimum must not be greater than maximum")
            }
            Self::UnsupportedImportKind => f.write_str("unsupported import kind"),
            Self::MissingValue => f.write_str("missing value")
        }
    }
}
