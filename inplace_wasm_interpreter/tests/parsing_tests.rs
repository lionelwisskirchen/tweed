use inplace_wasm_interpreter::{
    binary::{
        code_section::process_code_section, export_section::process_export_section,
        function_section::process_function_section, type_section::process_type_section,
    },
    core::wasm_reader::WasmReader,
    initialize,
};
use rstest::rstest;

#[rstest]
#[case(b"")] // No section size
#[case(b"\x01")] // No number of codes
#[case(b"\x01\x01")] // No number of locals
#[case(b"\x01\x01\x01")] // No func body size
#[case(b"\x01\x01\x01\x01")] // No count of local type
#[case(b"\x01\x01\x01\x01\x01")] // No local_type
#[case(b"\x01\x01\x01\x01\x01\x01")] // Invalid val type
fn test_code_section(#[case] bytes: &[u8]) {
    let mut reader = WasmReader::new(bytes.to_vec());
    let actual = process_code_section(&mut reader);
    assert!(actual.is_err(), "Expected an error but got: {:?}", actual);
}

#[rstest]
#[case(b"")] // No section size
#[case(b"\x01")] // No number of exports
#[case(b"\x01\x01")] // No length of export name
#[case(b"\x01\x01\x01")] // No export name
#[case(b"\x01\x01\x01\x01")] // No export kind
#[case(b"\x01\x01\x01\x01\x01")] // Invalid export kind
#[case(b"\x01\x01\x01\x01\x00")] // No index for function export kind
fn test_export_section(#[case] bytes: &[u8]) {
    let mut reader = WasmReader::new(bytes.to_vec());
    let actual = process_export_section(&mut reader);
    assert!(actual.is_err(), "Expected an error but got: {:?}", actual);
}

#[rstest]
#[case(b"")] // No section size
#[case(b"\x01")] // No number of types
#[case(b"\x01\x01")] // No length of type
#[case(b"\x01\x01\x01")] // No number of params
#[case(b"\x01\x01\x01\x01")] // No params
#[case(b"\x01\x01\x01\x01\x01\xFF")] // Invalid Params type
#[case(b"\x01\x01\x01\x00")] // No number of results
#[case(b"\x01\x01\x01\x00\x01")] // No results
#[case(b"\x01\x01\x01\x00\x01\xFF")] // Invalid Result type
fn test_type_section(#[case] bytes: &[u8]) {
    let mut reader = WasmReader::new(bytes.to_vec());
    let actual = process_type_section(&mut reader);
    assert!(actual.is_err(), "Expected an error but got: {:?}", actual);
}

#[rstest]
#[case(b"")] // No section size
#[case(b"\x01")] // No number of functions
#[case(b"\x01\x01")] // No func declaration
fn test_function_section(#[case] bytes: &[u8]) {
    let mut reader = WasmReader::new(bytes.to_vec());
    let actual = process_function_section(&mut reader);
    assert!(actual.is_err(), "Expected an error but got: {:?}", actual);
}
#[rstest]
#[case(b"\x01\x61")] // Invalid Magic Number
#[case(b"\x00\x61\x73\x6D\xFF")] // Invalid Version Number
#[case(b"\x00\x61\x73\x6D\x01\x00\x00\x00\x77")] // Invalid section code
#[case(b"\x00\x61\x73\x6D\x01\x00\x00\x00\x01")] // Invalid type section
#[case(b"\x00\x61\x73\x6D\x01\x00\x00\x00\x03")] // Invalid func section
#[case(b"\x00\x61\x73\x6D\x01\x00\x00\x00\x0a")] // Invalid code section
#[case(b"\x00\x61\x73\x6D\x01\x00\x00\x00\x07")] // Invalid export section
fn test_module_environment_creation(#[case] bytes: &[u8]) {
    let actual = initialize(bytes);
    assert!(actual.is_err(), "Expected an error but got: {:?}", actual);
}
