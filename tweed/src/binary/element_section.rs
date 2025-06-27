use crate::core::error::{Error, Result};
use crate::core::value_types::ValType;
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;
use alloc::vec::Vec;

use super::function_section::FuncDecl;
use super::import_section::GlobalImport;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Mode {
    Active {
        table: u32,
        offset_start: usize,
        offset_end: usize,
    },
    Passive,
    Declarative,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ElementItems {
    RefFuncs(Vec<u32>),
    Exprs(ValType, Vec<Span>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElementSegment {
    pub ref_type: ValType,
    pub mode: Mode,
    pub items: ElementItems,
}

pub fn process_element_section(
    reader: &mut WasmReader,
    global_imports: &[GlobalImport],
    functions: &[FuncDecl],
) -> Result<Vec<ElementSegment>> {
    reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    // Size is not required for decoding, so just check if size is present
    let mut elements = Vec::new();
    let number_of_element_segments = reader
        .next_leb128_u32()
        .ok_or(Error::InvalidSectionBytecode)?;
    for _ in 0..number_of_element_segments {
        let flag = reader
            .next_leb128_u32()
            .ok_or(Error::InvalidSectionBytecode)?;

        match flag {
            0 => {
                let start = reader.current_index();
                validate_const_expr(reader, global_imports, &[ValType::I32], functions)?; // offset expression
                let end = reader.current_index();
                let mode = Mode::Active {
                    table: 0,
                    offset_start: start,
                    offset_end: end,
                };
                let number_of_elements = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;

                let mut function_references = vec![];

                for _ in 0..number_of_elements {
                    let function_idx = reader
                        .next_leb128_u32()
                        .ok_or(Error::InvalidSectionBytecode)?;
                    if functions.get(function_idx as usize).is_none() {
                        return Err(Error::UnknownFunction);
                    }
                    function_references.push(function_idx);
                }

                elements.push(ElementSegment {
                    ref_type: ValType::FuncRef,
                    mode,
                    items: ElementItems::RefFuncs(function_references),
                });
            }

            1 => {
                let mode = Mode::Passive;
                let elemkind = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
                match elemkind {
                    0x00 => {
                        let number_of_elements = reader
                            .next_leb128_u32()
                            .ok_or(Error::InvalidSectionBytecode)?;

                        let mut function_references = vec![];
                        for _ in 0..number_of_elements {
                            let function_idx = reader
                                .next_leb128_u32()
                                .ok_or(Error::InvalidSectionBytecode)?;
                            function_references.push(function_idx);
                        }
                        elements.push(ElementSegment {
                            ref_type: ValType::FuncRef,
                            mode,
                            items: ElementItems::RefFuncs(function_references),
                        });
                    }
                    _ => return Err(Error::InvalidSectionBytecode), // TODO add proper error
                }
            }
            2 => {
                let tableidx = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let start = reader.current_index();
                validate_const_expr(reader, global_imports, &[ValType::I32], functions)?; // offset expression
                let end = reader.current_index();
                let mode = Mode::Active {
                    table: tableidx,
                    offset_start: start,
                    offset_end: end,
                };
                let elemkind = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
                match elemkind {
                    0x00 => {
                        let number_of_elements = reader
                            .next_leb128_u32()
                            .ok_or(Error::InvalidSectionBytecode)?;
                        let mut function_references = vec![];
                        for _ in 0..number_of_elements {
                            let function_idx = reader
                                .next_leb128_u32()
                                .ok_or(Error::InvalidSectionBytecode)?;

                            if functions.get(function_idx as usize).is_none() {
                                return Err(Error::UnknownFunction);
                            }

                            function_references.push(function_idx);
                        }
                        elements.push(ElementSegment {
                            ref_type: ValType::FuncRef,
                            mode,
                            items: ElementItems::RefFuncs(function_references),
                        });
                    }
                    _ => return Err(Error::InvalidSectionBytecode), // TODO add proper error
                }
            }
            3 => {
                let mode = Mode::Declarative;
                let elemkind = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
                match elemkind {
                    0x00 => {
                        let number_of_elements = reader
                            .next_leb128_u32()
                            .ok_or(Error::InvalidSectionBytecode)?;
                        let mut function_references = vec![];
                        for _ in 0..number_of_elements {
                            let function_idx = reader
                                .next_leb128_u32()
                                .ok_or(Error::InvalidSectionBytecode)?;
                            if functions.get(function_idx as usize).is_none() {
                                return Err(Error::UnknownFunction);
                            }
                            function_references.push(function_idx);
                        }
                        elements.push(ElementSegment {
                            ref_type: ValType::FuncRef,
                            mode,
                            items: ElementItems::RefFuncs(function_references),
                        });
                    }
                    _ => return Err(Error::InvalidSectionBytecode), // TODO add proper error
                }
            }
            4 => {
                let start = reader.current_index();
                validate_const_expr(reader, global_imports, &[ValType::I32], functions)?; // offset expression
                let end = reader.current_index();
                let mode = Mode::Active {
                    table: 0,
                    offset_start: start,
                    offset_end: end,
                };
                let number_of_elements = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let mut function_references = vec![];
                for _ in 0..number_of_elements {
                    let start = reader.current_index();
                    validate_const_expr(reader, global_imports, &[ValType::FuncRef], functions)?;
                    let end = reader.current_index();
                    function_references.push(Span { start, end })
                }
                elements.push(ElementSegment {
                    ref_type: ValType::FuncRef,
                    mode,
                    items: ElementItems::Exprs(ValType::FuncRef, function_references),
                });
            }
            5 => {
                let mode = Mode::Passive;
                let reftype_byte = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
                let reftype = ValType::lookup(reftype_byte)?;
                let number_of_elements = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let mut function_references = vec![];
                for _ in 0..number_of_elements {
                    let start = reader.current_index();
                    validate_const_expr(reader, global_imports, &[reftype], functions)?;
                    let end = reader.current_index();
                    function_references.push(Span { start, end })
                }
                elements.push(ElementSegment {
                    ref_type: reftype,
                    mode,
                    items: ElementItems::Exprs(reftype, function_references),
                });
            }
            6 => {
                let tableidx = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let start = reader.current_index();
                validate_const_expr(reader, global_imports, &[ValType::I32], functions)?; // offset expression
                let end = reader.current_index();
                let mode = Mode::Active {
                    table: tableidx,
                    offset_start: start,
                    offset_end: end,
                };
                let reftype_byte = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
                let reftype = ValType::lookup(reftype_byte)?;
                let number_of_elements = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let mut function_references = vec![];
                for _ in 0..number_of_elements {
                    let start = reader.current_index();
                    validate_const_expr(reader, global_imports, &[reftype], functions)?;
                    let end = reader.current_index();
                    function_references.push(Span { start, end })
                }
                elements.push(ElementSegment {
                    ref_type: reftype,
                    mode,
                    items: ElementItems::Exprs(reftype, function_references),
                });
            }
            7 => {
                let mode = Mode::Declarative;
                let reftype_byte = reader.next_byte().ok_or(Error::InvalidSectionBytecode)?;
                let reftype = ValType::lookup(reftype_byte)?;
                let number_of_elements = reader
                    .next_leb128_u32()
                    .ok_or(Error::InvalidSectionBytecode)?;
                let mut function_references = vec![];
                for _ in 0..number_of_elements {
                    let start = reader.current_index();
                    validate_const_expr(reader, global_imports, &[reftype], functions)?;
                    let end = reader.current_index();
                    function_references.push(Span { start, end })
                }
                elements.push(ElementSegment {
                    ref_type: reftype,
                    mode,
                    items: ElementItems::Exprs(reftype, function_references),
                });
            }
            _ => unreachable!(),
        };
    }
    Ok(elements)
}
