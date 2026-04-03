use okto::compiler::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let default_file = "examples/test.asm".to_string();
    let file = args.get(1).unwrap_or(&default_file).clone();

    match lex_and_process_file(&file) {
        Ok((ptkns, file_table)) => {
            println!("Tokens:");
            for t in &ptkns {
                println!("{:?} - {:?}", t.token, t.position);
            }
            println!("\nFile Table: {:#?}", file_table);

            //
            println!("===================================");
            match OktoAST::from_positioned_tokens(&ptkns) {
                Ok(ast) => {
                    for c in &ast.code {
                        match c {
                            OktoCodeItem::LabelsInstrRegImm(_, instr, reg, imm) => {
                                println!("{:?} {:?} {:?}", instr.token, reg.token, imm.token);
                            },
                            OktoCodeItem::LabelsInstrRegReg(_, instr, reg1, reg2) => {
                                println!("{:?} {:?} {:?}", instr.token, reg1.token, reg2.token);
                            },
                            OktoCodeItem::LabelsInstr(_, instr) => {
                                println!("{:?}", instr.token);
                            },
                            OktoCodeItem::LabelsInstrLabel(_, instr, label) => {
                                println!("{:?} {:?}", instr.token, label.token);
                            },
                        }
                    }
                }
                Err(e) => println!("Error: {}", e.error),
            }
        }
        Err(e) => {
            println!("Error: {}", e.error);
        }
    }

}
