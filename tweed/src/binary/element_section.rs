use alloc::collections::BTreeSet;

use crate::binary::table_section::Table;
use crate::core::error::ValidationError;
use crate::core::value_types::{RefType, ValType};
use crate::core::wasm_reader::WasmReader;
use crate::validation::constant_expression_validator::validate_const_expr;
use alloc::vec::Vec;

use super::module_environment::ModuleEnvironment;
pub type Result<T> = core::result::Result<T, ValidationError>;

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
/// Represents the items within an element segment
///
/// Element segments initialize tables with references (functions or other values)
/// at specific indices. This enum captures the two possible forms of element
/// initialization defined in the WebAssembly specification.
pub enum ElementItems {
    /// Initializes table entries with function references.
    ///
    /// Contains a list of function indices (index on the store) that will be written as **funcref**
    /// values into the table.
    RefFuncs(Vec<u32>),

    /// Initializes table entries with values computed by constant expressions.
    ///
    /// - The first field (`ValType`) specifies the type of values produced
    ///   by the expressions (e.g., `funcref`, `externref`).
    /// - The second field (`Vec<Span>`) contains the bytecode spans of the
    ///   constant expressions that will be evaluated to produce the values.
    ///
    /// Each expression in the vector is evaluated once, in order, and the
    /// resulting values are written to consecutive table indices.
    Exprs(ValType, Vec<Span>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElementSegment {
    pub ref_type: ValType,
    pub mode: Mode,
    pub items: ElementItems,
}

/// Processes the **element section** according to [WebAssembly specification: Element Section](https://webassembly.github.io/spec/core/binary/modules.html#element-section).
///
/// This function differentiates between the 9 segment types from [5.5.12. Element Section](https://www.w3.org/TR/2025/CRD-wasm-core-2-20250616/#element-section%E2%91%A0)
/// Depending on the segment type, the element section is interpreted differently.
///
/// # Arguments
/// * `reader`: a mutable reference to the WasmReader to read more bytes.
/// * `global_imports`: a reference to an array of global imported ValTypes.
/// * `functions`: a reference to already known function declarations.
/// * `tables`: a reference to the already known tables.
///
/// # Return
/// Ok(Vec<ElementSegment>) if everything is fine.
///
/// # Error
/// * `Error::UnknownTable`: The elem references an unknown table
/// * `Error::UnknownFunction`: The elem references an unknown function
pub fn process_element_section(
    reader: &mut WasmReader,
    module: &mut ModuleEnvironment,
    refs: &mut BTreeSet<usize>,
) -> Result<Vec<ElementSegment>> {
    let section_size = reader.next_leb128_u32()?;
    let section_end = reader.current_index() + section_size as usize;
    // Size is not required for decoding, so just check if size is present
    let mut elements = Vec::new();
    let number_of_element_segments = reader.next_leb128_u32()?;
    for _ in 0..number_of_element_segments {
        elements.push(process_element(reader, section_end, module, refs)?);
    }
    if reader.current_index() != section_end {
        return Err(ValidationError::SectionSizeMismatch);
    };
    Ok(elements)
}
fn process_element(
    reader: &mut WasmReader,
    section_end: usize,
    module: &mut ModuleEnvironment,
    refs: &mut BTreeSet<usize>,
) -> Result<ElementSegment> {
    let flag = reader.next_leb128_u32()?;
    let mut all_tables: Vec<Table> = Vec::new();
    all_tables.extend(module.importsec.tables.iter().map(|t| t.context.clone()));
    all_tables.extend(module.tablesec.iter().cloned());
    match flag {
        // No specific table is used
        // Take the first possible and instantiate an element in it
        0 => {
            let start = reader.current_index();

            validate_const_expr(
                reader,
                &[ValType::I32],
                section_end,
                module,
                module.globalsec.len(),
                refs,
            )?; // offset expression

            let end = reader.current_index();

            // no table idx is seen, so we check if there is an table at idx 0
            // if not we return an Error
            let table = all_tables.first().ok_or(ValidationError::UnknownTable)?;
            if table.ref_type != RefType::FuncRef {
                return Err(ValidationError::TypeMismatch);
            }
            let mode = Mode::Active {
                table: 0,
                offset_start: start,
                offset_end: end,
            };

            let number_of_elements = reader.next_leb128_u32()?;
            let mut function_references = vec![];

            for _ in 0..number_of_elements {
                let function_idx = reader.next_leb128_u32()?;
                if module.funcsec.get(function_idx as usize).is_none() {
                    return Err(ValidationError::UnknownFunction(None));
                }
                refs.insert(function_idx as usize);
                function_references.push(function_idx);
            }

            Ok(ElementSegment {
                ref_type: ValType::FuncRef,
                mode,
                items: ElementItems::RefFuncs(function_references),
            })
        }

        1 => {
            let mode = Mode::Passive;

            let elemkind = reader.next_byte()?;
            match elemkind {
                0x00 => {
                    let number_of_elements = reader.next_leb128_u32()?;

                    let mut function_references = vec![];

                    for _ in 0..number_of_elements {
                        let function_idx = reader.next_leb128_u32()?;
                        function_references.push(function_idx);
                        refs.insert(function_idx as usize);
                    }
                    Ok(ElementSegment {
                        ref_type: ValType::FuncRef,
                        mode,
                        items: ElementItems::RefFuncs(function_references),
                    })
                }
                _ => Err(ValidationError::InvalidElementKind),
            }
        }
        2 => {
            let tableidx = reader.next_leb128_u32()?;
            let start = reader.current_index();
            let table = all_tables
                .get(tableidx as usize)
                .ok_or(ValidationError::UnknownTable)?;
            if table.ref_type != RefType::FuncRef {
                return Err(ValidationError::TypeMismatch);
            };
            validate_const_expr(
                reader,
                &[ValType::I32],
                section_end,
                module,
                module.globalsec.len(),
                refs,
            )?; // offset expression
            let end = reader.current_index();
            let mode = Mode::Active {
                table: tableidx,
                offset_start: start,
                offset_end: end,
            };
            let elemkind = reader.next_byte()?;
            match elemkind {
                0x00 => {
                    let number_of_elements = reader.next_leb128_u32()?;
                    let mut function_references = vec![];
                    for _ in 0..number_of_elements {
                        let function_idx = reader.next_leb128_u32()?;

                        if module.funcsec.get(function_idx as usize).is_none() {
                            return Err(ValidationError::UnknownFunction(None));
                        }
                        refs.insert(function_idx as usize);
                        function_references.push(function_idx);
                    }
                    Ok(ElementSegment {
                        ref_type: ValType::FuncRef,
                        mode,
                        items: ElementItems::RefFuncs(function_references),
                    })
                }
                _ => Err(ValidationError::InvalidElementKind),
            }
        }
        3 => {
            let mode = Mode::Declarative;
            let elemkind = reader.next_byte()?;
            match elemkind {
                0x00 => {
                    let number_of_elements = reader.next_leb128_u32()?;
                    let mut function_references = vec![];
                    for _ in 0..number_of_elements {
                        let function_idx = reader.next_leb128_u32()?;
                        if module.funcsec.get(function_idx as usize).is_none() {
                            return Err(ValidationError::UnknownFunction(None));
                        }
                        refs.insert(function_idx as usize);
                        function_references.push(function_idx);
                    }
                    Ok(ElementSegment {
                        ref_type: ValType::FuncRef,
                        mode,
                        items: ElementItems::RefFuncs(function_references),
                    })
                }
                _ => Err(ValidationError::InvalidElementKind),
            }
        }
        4 => {
            let start = reader.current_index();
            validate_const_expr(
                reader,
                &[ValType::I32],
                section_end,
                module,
                module.globalsec.len(),
                refs,
            )?; // offset expression
            let table = all_tables.first().ok_or(ValidationError::UnknownTable)?;

            if table.ref_type != RefType::FuncRef {
                return Err(ValidationError::TypeMismatch);
            };
            let end = reader.current_index();
            let mode = Mode::Active {
                table: 0,
                offset_start: start,
                offset_end: end,
            };
            let number_of_elements = reader.next_leb128_u32()?;
            let mut function_references = vec![];
            for _ in 0..number_of_elements {
                let start = reader.current_index();
                validate_const_expr(
                    reader,
                    &[ValType::FuncRef],
                    section_end,
                    module,
                    module.globalsec.len(),
                    refs,
                )?;
                let end = reader.current_index();

                function_references.push(Span { start, end })
            }
            Ok(ElementSegment {
                ref_type: ValType::FuncRef,
                mode,
                items: ElementItems::Exprs(ValType::FuncRef, function_references),
            })
        }
        5 => {
            let mode = Mode::Passive;
            let reftype_byte = reader.next_byte()?;
            let reftype = ValType::lookup(&reftype_byte)?;
            let number_of_elements = reader.next_leb128_u32()?;
            let mut function_references = vec![];

            for _ in 0..number_of_elements {
                let start = reader.current_index();
                validate_const_expr(
                    reader,
                    &[reftype],
                    section_end,
                    module,
                    module.globalsec.len(),
                    refs,
                )
                .map_err(|e| match e {
                    ValidationError::TypeMismatch => ValidationError::MalformedRefType,
                    _ => e,
                })?;
                let end = reader.current_index();
                function_references.push(Span { start, end })
            }
            Ok(ElementSegment {
                ref_type: reftype,
                mode,
                items: ElementItems::Exprs(reftype, function_references),
            })
        }
        6 => {
            let tableidx = reader.next_leb128_u32()?;
            let start = reader.current_index();
            validate_const_expr(
                reader,
                &[ValType::I32],
                section_end,
                module,
                module.globalsec.len(),
                refs,
            )?; // offset expression
            let end = reader.current_index();
            let mode = Mode::Active {
                table: tableidx,
                offset_start: start,
                offset_end: end,
            };
            let reftype_byte = reader.next_byte()?;
            let reftype = ValType::lookup(&reftype_byte)?;
            let number_of_elements = reader.next_leb128_u32()?;
            let mut function_references = vec![];
            let table = all_tables
                .get(tableidx as usize)
                .ok_or(ValidationError::UnknownTable)?;

            if table.ref_type.to_val_type() != reftype {
                return Err(ValidationError::TypeMismatch);
            };

            for _ in 0..number_of_elements {
                let start = reader.current_index();
                validate_const_expr(
                    reader,
                    &[reftype],
                    section_end,
                    module,
                    module.globalsec.len(),
                    refs,
                )?;

                let end = reader.current_index();
                function_references.push(Span { start, end })
            }
            Ok(ElementSegment {
                ref_type: reftype,
                mode,
                items: ElementItems::Exprs(reftype, function_references),
            })
        }
        7 => {
            let mode = Mode::Declarative;
            let reftype_byte = reader.next_byte()?;
            let reftype = ValType::lookup(&reftype_byte)?;
            let number_of_elements = reader.next_leb128_u32()?;
            let mut function_references = vec![];
            for _ in 0..number_of_elements {
                let start = reader.current_index();
                validate_const_expr(
                    reader,
                    &[reftype],
                    section_end,
                    module,
                    module.globalsec.len(),
                    refs,
                )?;

                let end = reader.current_index();
                function_references.push(Span { start, end })
            }
            Ok(ElementSegment {
                ref_type: reftype,
                mode,
                items: ElementItems::Exprs(reftype, function_references),
            })
        }
        _ => unreachable!(),
    }
}
