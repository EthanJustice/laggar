use std::fs;
use std::path::Path;

// External crates
use html2md::parse_html;
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
		Ok(data) => parse_html(data.as_str()),
		Err(error) => panic!("Failed to download site: {}", error)
	};

	create_file(md, url);
}

fn get_site(url: String) -> Result<String, Box<dyn std::error::Error>> {
	let site = blocking::get(&url)?
		.text()?;
	
	Ok(site)
}

fn create_file(markdown: String, url: &str) -> std::io::Result<()> {
	if Path::new("./content/").is_dir() == false { create_directory() }

	let new_url = url.replace("https://", "").replace("http://", "").replace("/", ".");

	let mut path = String::from("content/") + &new_url + ".md";
	path = path.replace("..", ".");

	fs::write(path, markdown.as_bytes())?;

	Ok(())
}

fn create_directory() {
	fs::create_dir("./content").expect("Failed to create directory.");
}