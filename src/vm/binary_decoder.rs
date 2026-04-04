use std::collections::HashMap;
use crate::debug::*;

pub fn binary_decode(data: &Vec<u8>) -> Result<HashMap<String, Vec<u8>>, OktoError> {
    let mut cursor = 0usize;

    if data.len() < 7 {
        return Err("Binary too small".into());
    }

    if &data[0..4] != b"okto" {
        return Err("Invalid binary signature".into());
    }
    cursor += 4;

    let version = u16::from_be_bytes([data[cursor], data[cursor + 1]]);
    cursor += 2;

    if version != 1 {
        return Err(format!("Unsupported version: {}", version).into());
    }

    //---------------------------------------

    let section_quantity = data[cursor] as usize;
    cursor += 1;

    // read section headers first
    let mut section_headers: Vec<(String, usize)> = Vec::new();

    for _ in 0..section_quantity {
        let name_start = cursor;

        while cursor < data.len() && data[cursor] != 0 {
            cursor += 1;
        }

        if cursor >= data.len() {
            return Err("Unexpected end of file while reading section name".into());
        }

        let name_bytes = &data[name_start..cursor];
        let name = match String::from_utf8(name_bytes.to_vec()) {
            Ok(s) => s,
            Err(e) => return Err(format!("Invalid UTF-8 in section name: {}", e).into()),
        };

        cursor += 1; // skip '\0'

        if cursor + 1 >= data.len() {
            return Err("Unexpected end of file while reading section length".into());
        }

        let length = u16::from_be_bytes([data[cursor], data[cursor + 1]]) as usize;
        cursor += 2;

        section_headers.push((name, length));
    }

    //---------------------------------------

    let mut decoded_sections: HashMap<String, Vec<u8>> = HashMap::new();

    for (name, length) in section_headers {
        if cursor + length > data.len() {
            return Err(format!(
                "Section '{}' exceeds binary size (expected {} bytes)",
                name, length
            ).into());
        }

        let section_data = data[cursor..cursor + length].to_vec();
        cursor += length;

        if decoded_sections.insert(name.clone(), section_data).is_some() {
            return Err(format!("Duplicate section name: '{}'", name).into());
        }
    }

    //---------------------------------------

    if cursor != data.len() {
        return Err("Binary contains trailing unused bytes".into());
    }

    Ok(decoded_sections)
}