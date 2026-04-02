use okto::compiler::*;
use okto::debug;

fn main() {
    match std::fs::read_to_string("test.asm") {
        Ok(src) => {
            let mut positioned_tokens = Vec::new();

            lex_source_fn(
                src, 
                | res, line, column | {
                    match res {
                        AxolLexerCallbackResponse::Token(t) => match OktoToken::from(&t) {
                            Ok(tt) => positioned_tokens.push(tt),
                            Err(e) => debug::exit_compiler_with_error_and_position(&e, &"idk".to_string(), line, column),
                        }
                        AxolLexerCallbackResponse::String(s) => positioned_tokens.push(OktoToken::new_string_literal(&s)),
                        AxolLexerCallbackResponse::Char(c) => positioned_tokens.push(OktoToken::new_char_literal(&c)),
                    }
                }
            );

            println!("Positioned tokens: {:#?}", positioned_tokens);
        }
        Err(e) => println!("Error reading file: {}", e),
    };
}
