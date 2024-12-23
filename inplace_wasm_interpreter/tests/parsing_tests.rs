use inplace_wasm_interpreter::{
    binary::{code_section::process_code_section, export_section::process_export_section},
    core::wasm_reader::WasmReader,
};
use rstest::rstest;

#[rstest]
#[case(b"")]
#[case(b"\x01")]
#[case(b"\x01\x01")]
#[case(b"\x01\x01\x01")]
#[case(b"\x01\x01\x01\x01")]
#[case(b"\x01\x01\x01\x01\x01")]
#[case(b"\x01\x01\x01\x01\x01\x01")]
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
