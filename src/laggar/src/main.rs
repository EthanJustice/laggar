// External crates
use html2md::*;
// use crossterm::*;
use reqwest::blocking;
// use termimad::*;
use clap::{App, Arg};

fn main() {
	let clap = App::new("Laggar")
		.version("0.1.0")
		.author("Ethan Justice")
		.about("site to markdown converter")
		.arg(Arg::with_name("download")
			.short("d")
			.long("download")
			.help("Download and convert a website")
			.takes_value(true)
			.required(true)
		).get_matches();

	let url = clap.value_of("download").unwrap();
	
	let site = get_site(String::from(url));
	
	let md = match site {
		Ok(data) => html2md::parse_html(data.as_str()),
		Err(error) => panic!("Failed to download site: {}", error)
	};
}

fn get_site(url: String) -> Result<String, Box<dyn std::error::Error>> {
	println!("Getting!");
	let site = reqwest::blocking::get(&url)?
		.text()?;
	
	Ok(site)
}