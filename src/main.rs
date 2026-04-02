use okto::compiler::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let default_file = "examples/test.asm".to_string();
    let file = args.get(1).unwrap_or(&default_file).clone();

    match lex_and_process_file(&file) {
        Ok(tokens) => {
            println!("Tokens:");
            for t in tokens {
                println!("{:?}", t);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e.error);
        }
    }

}
