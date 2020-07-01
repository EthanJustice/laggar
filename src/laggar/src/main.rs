// External crates
use html2md::*;
use crossterm::*;
use reqwest::*;
use termimad::*;
use clap::*;

fn main() {
	let clap = clap::App::new("Laggar")
		.version("0.1.0")
		.author("Ethan Justice")
		.about("site to markdown converter")
		.arg(clap::Arg::with_name("download")
			.short("d")
			.long("download")
			.help("Download and convert a website")
			.takes_value(true)
			.required(true)
		).get_matches();

	let url = clap.value_of("download").unwrap();
}
