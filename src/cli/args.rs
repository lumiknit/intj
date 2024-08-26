use clap::{arg, command, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Compile a file")]
    Compile {
        #[arg(
            long,
            default_value_t = false,
            help = "Output the compiled file in Javascript"
        )]
        out_js: bool,

        #[arg(
            long,
            default_value_t = false,
            help = "Output the compiled file in GoLang"
        )]
        out_go: bool,

        #[arg(help = "The file to compile")]
        target: String,
    },

    #[command(about = "Start interactive mode")]
    Interactive {
        #[arg(
            short = 'f',
            long = "file",
            help = ".intj files to load before starting interactive mode"
        )]
        files: Vec<String>,
    },

    #[command(about = "Run a file")]
    Run {
        #[arg(help = ".intj file")]
        target: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    println!("{:?}", args);
    args
}
