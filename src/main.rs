pub mod cli;
pub mod token;

fn main() {
	let args = cli::parse_args();
	cli::run(args);
}
