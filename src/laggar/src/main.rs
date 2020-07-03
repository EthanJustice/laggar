#![allow(unused_must_use)]

// std imports
use std::fs;
use std::path::Path;
use std::io::{stdout, Write};
use std::process::exit;

// External crates
use html2md::parse_html;
use crossterm::{
	terminal::SetTitle,
	style::{ Print, style, Color, StyledContent, Attribute },
	execute,
	Result
};
use reqwest::blocking;
use clap::{App, Arg};

fn main() {
	set_status(style("Laggar:\n")); // Sets terminal window to "Laggar"
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

	let url = parse_url(String::from(clap.value_of("download").unwrap())); // Parsed url (inner is raw user input)
	
	set_status(style("Starting download...\n").with(Color::Cyan));
	
	let site = get_site(&url); // HTML from url

	set_status(style(&format!("Downloaded {}.\n", &url.as_str())[..]).with(Color::Yellow));

	let md = match site {
		Ok(data) => { // site scraping goes smoothly
			set_status(style("Parsing...\n").with(Color::Cyan));
			parse_html(data.as_str()) // converts html to markdown
		},
		Err(error) => { // site scraping fails
			set_status(style(&format!("\n\nFailed to download site.\nError: {}\n", error)[..]).with(Color::Red));
			exit(1) // exits process
		}
	};

	set_status(style("Creating file...\n").with(Color::Cyan));

	match create_file(md, &url) {
		// generating file/directory goes smoothly
		Ok(path) => set_status(style(&format!("Created file at {}.\n", Path::new(&path).display())[..]).with(Color::Yellow)),
		Err(error) => { // generating file/directory fails
			set_status(style(&format!("\n\nFailed to generate markdown.\nError: {}\n", error)[..]).with(Color::Red));
			exit(1) // exits process
		}
	};

	set_status(style("\nFinished successfully.\n").with(Color::Green).attribute(Attribute::Underlined));
}

fn parse_url(mut url: String) -> String { // Parses user-provided input
	if url.starts_with("http") == false {
		url.insert_str(0, "http://");
	}

	url
}

fn get_site(url: &String) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> { // Scrapes HTML from specified url
	let site = blocking::get(url)?
		.text()?;
	
	Ok(site)
}

fn create_file(markdown: String, url: &str) -> Result<String> { // Generates file (and "content" directory if necessary) and fills it with markdown from html
	if Path::new("./content/").is_dir() == false { create_directory() }

	let new_url = url.replace("https://", "").replace("http://", "").replace("/", ".");

	let mut path = String::from("content/") + &new_url + ".md";
	path = path.replace("..", ".");

	fs::write(&path, markdown.as_bytes())?;

	Ok(path)
}

fn create_directory() { // Creates directory if needed
	fs::create_dir("./content").expect("Failed to create directory.");
}

fn set_status(status: StyledContent<&str>) -> Result<()> { // Prints status to terminal; sets terminal window title
	execute!(
		stdout(),

		SetTitle(status.content()),

		Print(status)
	);

	Ok(())
}