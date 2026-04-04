use okto::cli::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.get(1).map(|s| s.as_str()) {
        Some("compiler") => {
            let mut cli = OktoCompilerCLI::new();
            cli.scan();
            cli.run();
        }
        _ => {
            let mut cli = OktoCLI::new();
            cli.scan();
            cli.run();
        }
    } 
}
