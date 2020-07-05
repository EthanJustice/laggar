pub struct Url {
	pub parsed: String, // fully-qualified url
	pub root: String, // root of url
	pub url_path: String // url converted to valid file path
}

impl Url {
	pub fn new(original: String) -> Url {
		let parsed_url = Url::parse(original.to_string());
		let url_root = Url::get_root(parsed_url.to_string());
		let url_path = Url::get_url_path(&parsed_url, &url_root);
		
		Url {
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
		let new_url = url.replace("http://", "").replace("https://", ""); // strips protocol
		let url_without_path: Vec<&str> = new_url.split("/").collect();

		let name: Vec<&str> = url_without_path[0].split(".").collect();

		format!("{}.{}", name[name.len() - 2], name[name.len() - 1]) // ignores subdomains
	}

	fn get_url_path(url: &String, domain: &String) -> String {
		url.to_string().replace("https://", "").replace("http://", "").replace("/", ".").replace("..", ".").replace(format!("{}.", domain).as_str(), "")
	}
}
