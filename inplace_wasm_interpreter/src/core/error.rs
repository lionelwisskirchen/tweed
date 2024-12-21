use core::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    InvalidMagic,
    InvalidVersion,
    InvalidSectionBytecode,
    TypeMismatch,
    InvalidSignatureIndex,
    InvalidControlFlow,
    MissingValue,
    InvalidLocalIndex,
    InvalidFunctionIndex,
    UndefinedOpCode(u8),
    UnknownLabel,
    UnknownSectionCode(u8),
    UnsupportedExportKind,
    UnsupportedValueType,
    ExpectedAValueOnTheStack,
    UnexpectedValueType,
    UnknownExportCall,
    InvalidSidetableIndex,
    UndefinedInstruction(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidMagic => {
                f.write_str("The magic number of the given Wasm file is invalid.")
            }
            Error::InvalidVersion => f.write_str("The version in the Wasm file is invalid"),
            Error::InvalidSectionBytecode => f.write_str("The section in the Wasm file is invalid"),
            Error::TypeMismatch | Error::InvalidSignatureIndex => f.write_str("type mismatch"),
            Error::InvalidControlFlow => f.write_str("Control flow mismatch"),
            Error::MissingValue => f.write_str("Missing value on stack."),
            Error::InvalidLocalIndex => f.write_str("Invalid local index"),
            Error::InvalidFunctionIndex => f.write_str("Invalid function index"),
            Error::UndefinedOpCode(opcode) => {
                f.write_fmt(format_args!("Invalid opcode {opcode:?}"))
            }
            Error::UndefinedInstruction(instruction) => {
                f.write_fmt(format_args!("Invalid instruction {instruction:?}"))
            }
            Error::UnknownSectionCode(seccode) => {
                f.write_fmt(format_args!("Invalid sectioncode {seccode:?}"))
            }
            Error::UnsupportedExportKind => f.write_str("unsupported export kind"),
            Error::UnknownLabel => f.write_str("unknown label"),
            Error::UnsupportedValueType => f.write_str("unknown value type"),
            Error::ExpectedAValueOnTheStack => f.write_str("No value on stack"),
            Error::UnexpectedValueType => f.write_str("Unexpected value type"),
            Error::UnknownExportCall => f.write_str("Unexpected export call"),
            Error::InvalidSidetableIndex => f.write_str("Invalid sidetable entry index"),
        }
    }
}
