use okto::compiler::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let default_file = "examples/test.asm".to_string();
    let file = args.get(1).unwrap_or(&default_file).clone();

    match lex_and_process_file(&file) {
        Ok((ptkns, _file_table)) => {
            match OktoAST::from_positioned_tokens(
                &ptkns, 
                &vec![
                    (".sprite".to_string(), OktoSectionType::Data),
                    (".audio".to_string(), OktoSectionType::Data)
                ],
            ) {
                Ok(mut ast) => {
                    let symbol_table = match resolve(&mut ast) {
                        Ok(st) => st,
                        Err(e) => {
                            println!("Error: {}", e.error);
                            return;
                        },
                    };
                    println!("\nSymbol Table: {:#?}", symbol_table);
                    println!("===================================");
                    //println!("{:#?}", ast);
                }
                Err(e) => println!("Error: {}, {:?}", e.error, e.position),
            }
        }
        Err(e) => {
            println!("Error: {}", e.error);
        }
    }

}
