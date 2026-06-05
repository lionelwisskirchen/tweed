use core::fmt;
use core::fmt::{Display, Formatter};
pub type Result<T> = core::result::Result<T, TweedError>;

/// A unified error type for the `Tweed` WebAssembly Runtime.
///
/// `TweedError` represents all errors that can occur while interpreting
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
pub enum TweedError {
    Validation(ValidationError),
    Execution(ExecutionError),
}

impl fmt::Display for TweedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TweedError::Validation(e) => write!(f, "{}", e),
            TweedError::Execution(e) => write!(f, "{}", e),
        }
    }
}

impl From<ValidationError> for TweedError {
    fn from(e: ValidationError) -> Self {
        TweedError::Validation(e)
    }
}

impl From<ExecutionError> for TweedError {
    fn from(e: ExecutionError) -> Self {
        TweedError::Execution(e)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExecutionError {
    InstanceNameTaken,
    IncompatibleNumberOfImports,
    UnknownReader,
    UnknownInstance,
    DivisionByZero,
    IntegerOverflow,
    MissingFunctionImport,
    MissingGlobalImport,
    MissingTableImport,
    MissingMemoryImport,
    IncompatibleImportType,
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
    MultipleMemories,
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InstanceNameTaken => f.write_str("instance name taken"),
            Self::UnknownReader => f.write_str("no reader found for instance"),
            Self::UnknownInstance => f.write_str("unknown instance"),
            Self::MissingGlobalImport => f.write_str("unknown import"),
            Self::MissingTableImport => f.write_str("unknown import"),
            Self::MissingFunctionImport => f.write_str("unknown import"),
            Self::MissingMemoryImport => f.write_str("unknown import"),
            Self::IncompatibleImportType => f.write_str("incompatible import type"),
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
            Self::MultipleMemories => f.write_str("multiple memories"),
            Self::IncompatibleNumberOfImports => f.write_str("incompatible number of imports"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValidationError {
    DuplicateExportName,
    OffsetOutOfRange,
    UnexpectedContentAfterLastSection,
    MalformedRefType,
    DataCountSectionRequired,
    ZeroByteExpected,
    InconsistentFunctionCodeSectionLengths,
    SectionSizeMismatch,
    ReadingOutOfBounds,
    IntegerTooLarge,
    TooManyLocals,
    InvalidIntegerRepresentation,
    InvalidStartFunctionSignature,
    NoDataSegmentFound(Option<usize>),
    UnknownMemoryIdx(usize),
    UnknownMemory,
    UnknownFunction(Option<usize>),
    UnknownType,
    UnknownTable,
    MultipleMemoriesNotImplemented,
    MultipleMemories,
    MalformedMutability,
    InvalidMagic,
    InvalidVersion,
    TypeMismatch,
    InvalidSignatureIndex,
    InvalidControlFlow,
    InvalidLocalIndex,
    InvalidFunctionIndex,
    UndefinedOpCode(u8),
    UndefinedConstantExpression,
    UnknownLabel,
    UnknownGlobal(Option<usize>),
    MalformedSectionId,
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
    MemorySizeExceeded,
    UnsupportedImportKind,
    InvalidStringRepresentation,
    UnequalChunkComparison,
    InvalidElementKind,
    MissingValue,
    UnexpectedEnd,
    UnexpectedEndOfFunctionOrSection,
    EndOpCodeExpected,
    InconsistentDataDataCountSectionLengths,
    LengthOutOfBounds,
    ConstantExpressionRequired,
    MalformedOneByteFlag,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OffsetOutOfRange => f.write_str("offset out of range"),

            Self::MalformedOneByteFlag => f.write_str("malformed limits flags"),
            Self::UnexpectedContentAfterLastSection => {
                f.write_str("unexpected content after last section")
            }
            Self::InconsistentFunctionCodeSectionLengths => {
                f.write_str("function and code section have inconsistent lengths")
            }
            Self::MalformedRefType => f.write_str("malformed reference type"),
            Self::InconsistentDataDataCountSectionLengths => {
                f.write_str("data count and data section have inconsistent lengths")
            }
            Self::DataCountSectionRequired => f.write_str("data count section required"),

            Self::TooManyLocals => f.write_str("too many locals"),
            Self::ZeroByteExpected => f.write_str("zero byte expected"),
            Self::SectionSizeMismatch => f.write_str("section size mismatch"),
            Self::InvalidStringRepresentation => f.write_str("invalid string representation"),
            Self::UnequalChunkComparison => f.write_str("unequal chunk comparison"),
            Self::ReadingOutOfBounds => f.write_str("reading out of bounds"),
            Self::LengthOutOfBounds => f.write_str("length out of bounds"),

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
            Self::DuplicateExportName => f.write_str("duplicate export name"),
            Self::MalformedMutability => f.write_str("malformed mutability"),
            Self::InvalidMagic => f.write_str("magic header not detected"),
            Self::GlobalIsImmutable => f.write_str("global is immutable"),
            Self::InvalidIntegerRepresentation => f.write_str("integer representation too long"),
            Self::InvalidVersion => f.write_str("unknown binary version"),
            Self::TypeMismatch | Self::InvalidSignatureIndex => f.write_str("type mismatch"),
            Self::InvalidControlFlow => f.write_str("invalid control flow"),
            Self::InvalidLocalIndex => f.write_str("unknown local"),
            Self::UnknownFunction(idx) => {
                if let Some(i) = idx {
                    f.write_fmt(format_args!("unknown function {i}"))
                } else {
                    f.write_str("unknown function")
                }
            }

            Self::InvalidFunctionIndex => f.write_str("invalid function index"),
            Self::UndefinedConstantExpression => f.write_str("illegal opcode"),
            Self::ConstantExpressionRequired => f.write_str("constant expression required"),

            Self::MultipleMemoriesNotImplemented => {
                f.write_str("multiple Memory proposal is not implemented")
            }
            Self::MultipleMemories => f.write_str("multiple memories"),
            Self::UndefinedOpCode(opcode) => {
                f.write_fmt(format_args!("illegal opcode {opcode:x?}"))
            }
            Self::MalformedSectionId => f.write_str("malformed section id"),
            Self::UnknownType => f.write_str("unknown type"),

            Self::UnknownGlobal(idx) => {
                if let Some(i) = idx {
                    f.write_fmt(format_args!("unknown global {i}"))
                } else {
                    f.write_str("unknown global")
                }
            }
            Self::UnknownTable => f.write_str("unknown table"),
            Self::UnsupportedExportKind => f.write_str("unsupported export kind"),
            Self::UnknownLabel => f.write_str("unknown label"),
            Self::UnsupportedValueType => f.write_str("unknown value type"),
            Self::IllegalSelectType => f.write_str("type mismatch"),

            Self::UndefinedMemory => f.write_str("undefined memory"),
            Self::UndefinedTable => f.write_str("undefined table"),
            Self::UnknownMemoryIdx(idx) => f.write_fmt(format_args!("unknown memory {idx}")),
            Self::UnknownMemory => f.write_fmt(format_args!("unknown memory")),
            Self::InvalidAlignment => f.write_str("alignment must not be larger than natural"),

            Self::InvalidElementKind => f.write_str("invalid element kind"),
            Self::SizeMinimumMustNotBeGreaterThanMaximum => {
                f.write_str("size minimum must not be greater than maximum")
            }
            Self::UnsupportedImportKind => f.write_str("malformed import kind"),
            Self::MissingValue => f.write_str("missing value"),
            Self::MemorySizeExceeded => {
                f.write_str("memory size must be at most 65536 pages (4GiB)")
            }
            Self::UnexpectedEnd => f.write_str("unexpected end"),
            Self::UnexpectedEndOfFunctionOrSection => {
                f.write_str("unexpected end of section or function")
            }

            Self::EndOpCodeExpected => f.write_str("END opcode expected"),
        }
    }
}
