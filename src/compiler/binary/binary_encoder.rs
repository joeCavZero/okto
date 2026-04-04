/*
    "okto" - 4 bytes
    <version> - 2 bytes
    <section quantity> - 1 byte, e.g.: 2
        <section 1 name zero ended> - n bytes, e.g.: .code
        <section 1 lenght> - 2 bytes
        <section 2 name zero ended> - n bytes, e.g.: .sprite
        <section 2 lenght> - 2 bytes
    <data of sections> 
*/

use crate::debug::OktoError;

pub fn binary_encode(version: u16, sections: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, OktoError> {
    let okto_bytes = b"okto";
    let version_bytes = version.to_be_bytes();
    let section_quantity_bytes = match u8::try_from(sections.len()) {
        Ok(l) => l,
        Err(e) => return Err(format!("Could not encode section quantity: {}", e).into())
    };
    let mut sections_bytes: Vec<u8> = Vec::new();
    let mut data_bytes: Vec<u8> = Vec::new();
    for (name, data) in sections {
        let mut name_bytes = name.into_bytes();
        name_bytes.push(0); // Null-terminated
        let length_bytes = match u16::try_from(data.len()) {
            Ok(l) => l.to_be_bytes(),
            Err(e) => return Err(format!("Could not encode section length: {}", e).into())
        };
        sections_bytes.extend(name_bytes);
        sections_bytes.extend(length_bytes);
        data_bytes.extend(data);
    }

    let mut binary = Vec::new();
    binary.extend(okto_bytes);
    binary.extend(version_bytes);
    binary.push(section_quantity_bytes);
    binary.extend(sections_bytes);
    binary.extend(data_bytes);
    Ok(binary)
}