use crate::core::error::ValidationError;

use crate::core::wasm_reader::WasmReader;
pub type Result<T> = core::result::Result<T, ValidationError>;

/// Describes the dependency and implementation view of a module.
///
/// This structure captures both:
/// - the interfaces a module depends on (imports)
/// - the interfaces it implements (exports)
#[derive(Debug, Default, Clone)]
pub struct LinkingInformation {
    pub package_name: String,
    pub depends: Vec<InterfaceId>,
    pub implements: Vec<InterfaceSpec>,
}

/// Represents a WIT interface specification.
///
/// This is a parsed, unresolved definition originating from the module's
/// custom section. It contains the interface identity and the declared
/// function signatures (by name only, before binding to actual function addresses).
#[derive(Debug, Default, Clone)]
pub struct InterfaceSpec {
    pub interface: InterfaceId,
    pub functions: Vec<FunctionName>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionName(pub String);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterfaceKey {
    pub namespace: String,
    pub package: String,
    pub interface: String,
}

/// Uniquely identifies a WIT interface within a package.
///
/// An InterfaceId is the canonical identifier used for linking,
/// resolution, and dependency tracking across modules.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterfaceId {
    pub key: InterfaceKey,
    pub version: String,
}
impl InterfaceId {
    pub fn interface_name(&self) -> String {
        format!(
            "{}:{}:{}@{}",
            self.key.namespace, self.key.package, self.key.interface, self.version
        )
    }
}
impl TryFrom<String> for InterfaceId {
    type Error = ValidationError;
    /// Parses an `Interface` from a string representation.
    ///
    /// The expected format is:
    /// `namespace:package:interface@version`
    fn try_from(value: String) -> Result<Self> {
        let (name, version) = value
            .split_once('@')
            .ok_or(ValidationError::MalformedInterfaceDefinition)?;

        let parts: Vec<_> = name.split(':').collect();

        let [namespace, package, interface] = parts.as_slice() else {
            return Err(ValidationError::MalformedInterfaceDefinition);
        };

        Ok(Self {
            key: InterfaceKey {
                namespace: (*namespace).into(),
                package: (*package).into(),
                interface: (*interface).into(),
            },
            version: version.into(),
        })
    }
}

pub fn process_custom_section(reader: &mut WasmReader) -> Result<Option<LinkingInformation>> {
    let section_size = reader.next_leb128_u32()? as usize;
    let start_of_section = reader.current_index();

    // Ensure the declared section size does not exceed remaining input.
    if reader.rest().len() < section_size {
        return Err(ValidationError::LengthOutOfBounds);
    }

    // We require at least 1 byte to begin parsing the custom section payload,
    // because the first field is a LEB128-encoded string length.
    if section_size == 0 {
        return Err(ValidationError::UnexpectedEnd);
    }

    let length_name = reader.next_leb128_u32()?;
    let name = reader.next_chunk_as_string(length_name as usize)?;
    let mut implements: Vec<InterfaceSpec> = Vec::new();
    let mut depends: Vec<InterfaceId> = Vec::new();
    let mut result = None;
    let mut package_name = "".to_string();
    if name == "dynamic_linking_information" {
        let len_to_end = section_size - (reader.current_index() - start_of_section);
        let content = reader.next_chunk_as_string(len_to_end)?;

        let mut parts = content.split_whitespace();

        while let Some(part) = parts.next() {
            match part {
                "implements" => {
                    if let Some(val) = parts.next() {
                        let fqwi = val.to_string();
                        let interface: InterfaceId = fqwi.try_into()?;
                        let mut functions = vec![];
                        for function in parts.by_ref() {
                            if function == ";" {
                                break;
                            }
                            functions.push(FunctionName(function.to_string()));
                        }
                        package_name =
                            format!("{:?}:{:?}", interface.key.namespace, interface.key.package);
                        implements.push(InterfaceSpec {
                            interface,
                            functions,
                        });
                    }
                }
                "depends" => {
                    depends = parts
                        .map(|s| InterfaceId::try_from(s.to_string()))
                        .collect::<Result<Vec<InterfaceId>>>()?;
                    break;
                }
                _ => {}
            }
        }
        if implements.is_empty() {
            return Err(ValidationError::MalformedInterfaceDefinition);
        }
        result = Some(LinkingInformation {
            depends,
            implements,
            package_name,
        });
    };

    if reader.current_index() > start_of_section + section_size {
        return Err(ValidationError::SectionSizeMismatch);
    }

    // Skip any remaining bytes in the section.
    // We intentionally ignore custom section payload contents.
    reader.seek(start_of_section + section_size + 1);

    Ok(result)
}
