#![allow(unused_must_use)]

// std
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::exit;
use std::time::Instant;

// external
use clap::{App, Arg};
use crossterm::{
    execute,
    style::{style, Attribute, Color, Print, StyledContent},
    terminal::SetTitle,
    Result,
}; // tui graphics
use html2md::parse_html; // html -> markdown conversion
use reqwest::blocking; // web requests // cli arg parsing

// Local
use laggar::Url;

fn main() {
    let clap = App::new("laggar")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("site to markdown converter")
        .arg(
            Arg::with_name("download")
                .short("d")
                .long("download")
                .help("Download and convert a website")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let timing = Instant::now();

    let url = Url::new(String::from(clap.value_of("download").unwrap())); // Parsed url (inner is raw user input)

    set_status(style("Starting download...\n").with(Color::Cyan));
    download(url);
    set_status(
        style(
            &format!(
                "\nFinished successfully in {}ms.\n",
                timing.elapsed().as_millis()
            )[..],
        )
        .with(Color::Green)
        .attribute(Attribute::Underlined),
    );
}

// downloads, converts, and saves the specified url
fn download(url: Url) {
    let download_timing = Instant::now();
    let site = get_site(&url.parsed); // HTML from url

    set_status(
        style(
            &format!(
                "Downloaded {} in {}ms\n",
                &url.parsed.as_str(),
                download_timing.elapsed().as_millis()
            )[..],
        )
        .with(Color::Yellow),
    );

    let parse_timing = Instant::now();
    let md = match site {
        Ok(data) => {
            // site scraping goes smoothly
            set_status(style("Parsing...\n").with(Color::Cyan));
            parse_html(data.as_str()) // converts html to markdown
        }
        Err(error) => {
            // site scraping fails
            set_status(
                style(&format!("\n\nFailed to download site.\nError: {}\n", error)[..])
                    .with(Color::Red),
            );
            exit(1) // exits process
        }
    };
    set_status(
        style(&format!("Parsed in {}ms\n", parse_timing.elapsed().as_millis())[..])
            .with(Color::Yellow),
    );

    set_status(style("Creating file...\n").with(Color::Cyan));

    let file_create_timing = Instant::now();
    match create_file(md, &url) {
        // generating file/directory goes smoothly
        Ok(path) => set_status(
            style(
                &format!(
                    "Created file at {} in {}ms\n",
                    Path::new(&path).display(),
                    file_create_timing.elapsed().as_millis()
                )[..],
            )
            .with(Color::Yellow),
        ),
        Err(error) => {
            // generating file/directory fails
            set_status(
                style(&format!("\n\nFailed to generate markdown.\nError: {}\n", error)[..])
                    .with(Color::Red),
            );
            exit(1) // exits process
        }
    };
}

// Scrapes HTML from specified url, explicit Result types needed due to reqwest's Result
fn get_site(url: &String) -> std::result::Result<String, std::boxed::Box<dyn std::error::Error>> {
    let site = blocking::get(url)?.text()?;

    Ok(site)
}

// Generates file (and "content" directory if necessary) and fills it with markdown from html
fn create_file(markdown: String, url: &Url) -> Result<String> {
    if Path::new("./content/").is_dir() == false {
        create_directory(String::from("./content/"))
    }
    if Path::new(&format!("./content/{}", url.root)).is_dir() == false {
        create_directory(format!("./content/{}", url.root))
    }

    let mut url_path = url.url_path.clone();
    if url_path == String::from("") {
        url_path = String::from("ROOT.")
    }

    let path = format!("./content/{}/{}md", url.root, url_path);
    fs::write(&path, markdown.as_bytes())?;

    Ok(path) // returned to print path
}

// Creates directory if needed
fn create_directory(path: String) {
    set_status(
        style(&format!("Creating directory at {}\n", Path::new(&path).display())[..])
            .with(Color::Yellow),
    );

    fs::create_dir(path).expect("Failed to create directory.");
}

// Prints status to terminal; sets terminal window title
fn set_status(status: StyledContent<&str>) -> Result<()> {
    execute!(stdout(), SetTitle(status.content()), Print(status));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn check_download() {
        download(Url::new(String::from("http://example.com/test")));
        let value = read_to_string(Path::new("content/example.com/test.md"))
            .expect("Failed to read output directory.");

        assert_eq!(value, String::from(" Example Domain  body { background\\-color: #f0f0f2; margin: 0; padding: 0; font\\-family: \\-apple\\-system, system\\-ui, BlinkMacSystemFont, \"Segoe UI\", \"Open Sans\", \"Helvetica Neue\", Helvetica, Arial, sans\\-serif; } div { width: 600px; margin: 5em auto; padding: 2em; background\\-color: #fdfdff; border\\-radius: 0.5em; box\\-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02); } a:link, a:visited { color: #38488f; text\\-decoration: none; } @media (max\\-width: 700px) { div { margin: 0 auto; width: auto; } }\n\nExample Domain\n==========\n\nThis domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.\n\n[More information...](https://www.iana.org/domains/example)\n\n"));

        // cleanup
        std::fs::remove_dir_all(Path::new("content"));
    }
}
