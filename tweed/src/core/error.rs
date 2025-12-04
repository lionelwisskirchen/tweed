use core::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ValidationError {
    ReadingOutOfBounds,
    IntegerTooLarge,
    InvalidIntegerRepresentation,
    TrapUnreachable,
    InvalidStartFunctionSignature,
    NoDataSegmentFound(Option<usize>),
    UnknownMemory(usize),
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
    IllegalSelectType,
    UndefinedMemory,
    UndefinedTable,
    TableAccessOutOfBounds,
    InvalidAlignment,
    MemoryAccessOutOfBounds,
    DivisionByZero,
    UnrepresentableResult,
    InvalidResultArity,
    GlobalIsImmutable,
    IntegerOverflow,
    InstanceNameTaken,
    ConstIsMutable,
    MalformedMemOpFlags,
    InvalidStringRepresentation,
    UnequalChunkComparison,
    InvalidElementKind,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidStringRepresentation => f.write_str("InvalidStringRepresentation"),
            Self::UnequalChunkComparison => f.write_str("UnequalChunkComparison"),
            Self::ReadingOutOfBounds => f.write_str("Reader out of bounds"),
            Self::MalformedMemOpFlags => f.write_str("malformed memop flags"),
            Self::TrapUnreachable => f.write_str("unreachable"),
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
            Self::InvalidConversionToInt => f.write_str("invalid conversion to integer"),
            Self::StackExhaustion => f.write_str("operand stack exhausted"),
            Self::CallStackExhaustion => f.write_str("call stack exhausted"),
            Self::MalformedMutability => f.write_str("malformed mutability"),
            Self::InvalidMagic => {
                f.write_str("The magic number of the given Wasm file is invalid.")
            }
            Self::InstanceNameTaken => f.write_str("instance name is taken"),
            Self::IndirectCallTypeMismatch => f.write_str("indirect call type mismatch"),
            Self::FunctionNotFound => f.write_str("function not found"),
            Self::GlobalIsImmutable => f.write_str("global is immutable"),
            Self::ConstIsMutable => f.write_str("global const is mutable"),

            Self::DivisionByZero => f.write_str("integer divide by zero"),
            Self::InvalidIntegerRepresentation => f.write_str("integer representation too long"),
            Self::IntegerOverflow => f.write_str("integer overflow"),
            Self::UnrepresentableResult => f.write_str("Result is not representable."),
            Self::InvalidVersion => f.write_str("The version in the Wasm file is invalid"),
            Self::InvalidSectionBytecode => f.write_str("The section in the Wasm file is invalid"),
            Self::TypeMismatch | Self::InvalidSignatureIndex => f.write_str("type mismatch"),
            Self::InvalidControlFlow => f.write_str("Control flow mismatch"),
            Self::MissingValue => f.write_str("Missing value on stack."),
            Self::InvalidLocalIndex => f.write_str("unknown local"),
            Self::UnknownFunction => f.write_str("unknown function"),

            Self::UndefinedElement => f.write_str("undefined element"),
            Self::InvalidFunctionIndex => f.write_str("Invalid function index"),
            Self::InvalidMemoryIndex => f.write_str("Invalid memory index"),
            Self::UndefinedContantExpression => f.write_str("constant expression required"),
            Self::MultipleMemoriesNotImplemented => {
                f.write_str("Multiple Memory proposal is not implemented")
            }
            Self::UndefinedOpCode(opcode) => {
                f.write_fmt(format_args!("Invalid opcode {opcode:#04X}"))
            }
            Self::UndefinedInstruction(instruction) => {
                f.write_fmt(format_args!("Invalid instruction {instruction:#04X}"))
            }
            Self::UnknownSectionCode(seccode) => {
                f.write_fmt(format_args!("Invalid sectioncode {seccode:#04X}"))
            }
            Self::UnknownType => f.write_str("unknown type"),

            Self::UnknownGlobal => f.write_str("unknown global"),
            Self::UnknownTable => f.write_str("unknown table"),
            Self::UninitializedReference => f.write_str("uninitialized element"),
            Self::UnsupportedExportKind => f.write_str("unsupported export kind"),
            Self::UnknownLabel => f.write_str("unknown label"),
            Self::UnsupportedValueType => f.write_str("unknown value type"),
            Self::ExpectedAValueOnTheStack => f.write_str("No value on stack"),
            Self::UnexpectedValueType => f.write_str("Unexpected value type"),
            Self::UnknownExportCall => f.write_str("Unexpected export call"),
            Self::InvalidSidetableIndex => f.write_str("Invalid sidetable entry index"),
            Self::IllegalSelectType => f.write_str("type mismatch"),
            Self::MissingGlobalImport => f.write_str("missing global import"),
            Self::MissingFunctionImport => f.write_str("missing function import"),
            Self::UndefinedMemory => f.write_str("undefined memory"),
            Self::UndefinedTable => f.write_str("undefined table"),
            Self::UnknownMemory(idx) => f.write_fmt(format_args!("unknown memory {idx}")),
            Self::InvalidAlignment => f.write_str("alignment must not be larger than natural"),
            Self::MemoryAccessOutOfBounds => f.write_str("out of bounds memory access"),
            Self::TableAccessOutOfBounds => f.write_str("out of bounds table access"),
            Self::UnknownInstance => f.write_str("unknown instance"),
            Self::IncompatibleImportType => f.write_str("incompatible import type"),
            Self::InvalidElementKind => f.write_str("InvalidElementKind"),
        }
    }
}
