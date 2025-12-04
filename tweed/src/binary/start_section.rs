use crate::core::error::Result;
use crate::core::error::ValidationError;
use crate::core::signature_lookup::lookup_signature;
use crate::core::wasm_reader::WasmReader;

use super::module_environment::ModuleEnvironment;

pub fn process_start_section(
    reader: &mut WasmReader,
    environment: &ModuleEnvironment,
) -> Result<u32> {
    reader.next_leb128_u32()?;
    // Size is not required for decoding, so just check if size is present
    let start_function_idx = reader.next_leb128_u32()?;

    let signature_index = environment
        .funcsec
        .get(start_function_idx as usize)
        .ok_or(ValidationError::UnknownFunction)?;
    let signature = lookup_signature(signature_index.sig_index, &environment.typesec);
    if !signature.parameter.is_empty() || !signature.result.is_empty() {
        return Err(ValidationError::InvalidStartFunctionSignature);
    }
    Ok(start_function_idx)
}
