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

struct Url {
	original: String, // original url passed by calling Url::new()
	parsed: String, // fully-qualified url
	root: String, // root of url
	url_path: String // url converted to valid file path
}

impl Url {
	fn new(original: String) -> Url {
		let parsed_url = Url::parse(original.to_string());
		let url_root = Url::get_root(parsed_url.to_string());
		let url_path = Url::get_url_path(&parsed_url);
		
		Url {
			original: original.to_string(),
			parsed: parsed_url,
			root: url_root,
			url_path: url_path
		}
	}

	fn parse(mut url: String) -> String {// Parses user-provided input
		if url.starts_with("http") == false { // not a fully-qualified url, doesn't contain http/https
			url.insert_str(0, "http://"); // in case sites don't have https
		}

		if url.ends_with("/") == false {
			url.push_str("/");
		}

		url
	}

	fn get_root(url: String) -> String {
		let new_url = url.replace("http://", "").replace("https://", "");
		let url_without_path: Vec<&str> = new_url.split("/").collect();

		let name: Vec<&str> = url_without_path[0].split(".").collect();

		format!("{}.{}", name[name.len() - 2], name[name.len() - 1]) // ignores subdomains
	}

	fn get_url_path(url: &String) -> String {
		url.to_string().replace("https://", "").replace("http://", "").replace("/", ".").replace("..", ".")
	}
}

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

	let url = Url::new(String::from(clap.value_of("download").unwrap())); // Parsed url (inner is raw user input)
	
	set_status(style("Starting download...\n").with(Color::Cyan));
	
	let site = get_site(&url.parsed); // HTML from url

	set_status(style(&format!("Downloaded {}.\n", &url.original.as_str())[..]).with(Color::Yellow));

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

	match create_file(md, &url.url_path) {
		// generating file/directory goes smoothly
		Ok(path) => set_status(style(&format!("Created file at {}.\n", Path::new(&path).display())[..]).with(Color::Yellow)),
		Err(error) => { // generating file/directory fails
			set_status(style(&format!("\n\nFailed to generate markdown.\nError: {}\n", error)[..]).with(Color::Red));
			exit(1) // exits process
		}
	};

	set_status(style("\nFinished successfully.\n").with(Color::Green).attribute(Attribute::Underlined));
}

fn get_site(url: &String) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> { // Scrapes HTML from specified url
	let site = blocking::get(url)?
		.text()?;
	
	Ok(site)
}

fn create_file(markdown: String, url: &str) -> Result<String> { // Generates file (and "content" directory if necessary) and fills it with markdown from html
	if Path::new("./content/").is_dir() == false { create_directory() }
	let path = String::from("content/") + url + ".md";

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