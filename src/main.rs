pub mod cli;
pub mod token;
pub mod utils;

fn main() {
	let args = cli::parse_args();
	cli::run(args);
}
