use crate::debug::*;
use crate::vm::*;

impl OktoVM {
    pub fn load_memory(&mut self, file_path: &String) -> Result<(), OktoError> {
        let data = match std::fs::read(file_path) {
            Ok(raw) => raw,
            Err(e) => return Err(format!("Failed to read file {}: {}", file_path, e)),
        };
        
        match binary_decode(&data) {
            Ok(mem) => {
                //let code_section = mem.get("code")
                //for code_byte in mem.get
                println!("{:#?}", mem);
            }
            Err(e) => return Err(format!("Failed to decode binary from file {}: {}", file_path, e).into()),
        }
        Ok(())
    }
}