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

	fn get_root(url: String) -> String { // gets root of provided url
		let new_url = url.replace("http://", "").replace("https://", ""); // strips protocol
		let url_without_path: Vec<&str> = new_url.split("/").collect();

		let name: Vec<&str> = url_without_path[0].split(".").collect();

		format!("{}.{}", name[name.len() - 2], name[name.len() - 1]) // ignores subdomains
	}

	fn get_url_path(url: &String, domain: &String) -> String { // formats url path to file system compatible version
		url.to_string().replace("https://", "").replace("http://", "").replace("/", ".").replace("..", ".").replace(format!("{}.", domain).as_str(), "")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn check_urls() {
		let domain_no_trail = Url::new(String::from("https://example.com"));
		let domain_no_prot = Url::new(String::from("example.com/about"));
		let domain_no_prot_no_trail = Url::new(String::from("example.com"));
		let sub_domain_no_trail = Url::new(String::from("https://example.example.com/")); // not an actual subdomain, but works for tests
		
		// parsed
		assert_eq!(domain_no_trail.parsed, String::from("https://example.com/"));
		assert_eq!(domain_no_prot.parsed, String::from("http://example.com/about/"));
		assert_eq!(domain_no_prot_no_trail.parsed, String::from("http://example.com/"));
		assert_eq!(sub_domain_no_trail.parsed, String::from("https://example.example.com/"));

		// roots
		assert_eq!(domain_no_trail.root, String::from("example.com"));
		assert_eq!(domain_no_prot.root, String::from("example.com"));
		assert_eq!(domain_no_prot_no_trail.root, String::from("example.com"));
		assert_eq!(sub_domain_no_trail.root, String::from("example.com"));

		// url_paths
		assert_eq!(domain_no_trail.url_path, String::from(""));
		assert_eq!(domain_no_prot.url_path, String::from("about."));
		assert_eq!(domain_no_prot_no_trail.url_path, String::from(""));
		assert_eq!(sub_domain_no_trail.url_path, String::from("example."));
	}
}