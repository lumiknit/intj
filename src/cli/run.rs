use super::{Args, Command};

pub fn run(args: Args) {
    match args.command {
        Command::Compile {
            out_js,
            out_go,
            target,
        } => {
            println!("Compiling file: {}", target);
            if out_js {
                println!("Outputting in Javascript");
            }
            if out_go {
                println!("Outputting in GoLang");
            }
        }
        Command::Interactive { files } => {
            println!("Starting interactive mode");
            if !files.is_empty() {
                println!("Loading files: {:?}", files);
            }
        }
        Command::Run { target } => {
            println!("Running file: {}", target);
        }
    }
}
