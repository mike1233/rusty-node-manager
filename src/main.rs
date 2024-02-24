mod utils;
use clap::Parser;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
#[clap(name = "Node.js installer", version = "1.0", author = "Mike Schepers")]

struct Args {
    /// Version of Node to install
    #[arg(short, long)]
    version_to_install: Option<String>,

    /// List available versions
    #[clap(short, long, default_value = "false")]
    list: bool,
}

fn main() {
    let args = Args::parse();
    let base_node_url: &str = "https://nodejs.org/dist/";

    if args.list {
        list_versions(base_node_url);
        return;
    }

    if args.version_to_install.is_none() {
        println!("No version specified");
        return;
    }

    let version = args.version_to_install.as_ref().unwrap();

    match version.as_str() {
        "latest" => {
            println!("Installing latest version of Node.js");
        }
        _ => {
            let res = find_installable_version(base_node_url, version.as_str());
        }
    }
}

fn list_versions(base_node_url: &str) -> () {
    let all_versions = get_all_versions(base_node_url);

    let mut versions: Vec<String> = all_versions.unwrap();
    versions.sort_by(|a, b| utils::node_version_compare_fn(a, b));

    println!("Available versions of Node.js:");
    for v in versions {
        println!("{}", v);
    }
}

fn get_all_versions(base_node_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response: reqwest::blocking::Response = reqwest::blocking::get(base_node_url)?;
    let body: String = response.text()?;

    let document: Html = Html::parse_document(&body);
    let selector: Selector = Selector::parse("a").unwrap();

    let mut all_versions: Vec<&str> = Vec::new();

    for node in document.select(&selector) {
        let item: Vec<&str> = node.text().collect::<Vec<_>>();

        if item.len() > 0 && item[0].starts_with("v") {
            all_versions.push(item[0]);
        }
    }

    let filtered_versions: Vec<String> = all_versions
        .iter()
        .filter(|v| v.contains("v"))
        .map(|v| v.replace("/", ""))
        .collect::<Vec<_>>();

    Ok(filtered_versions)
}

fn find_installable_version(
    base_node_url: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // verify version is a valid versionstring, allowed version strings are 1.2.3 or 10.11.12
    println!("Checking version string {}", version);

    let re: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();

    if !re.is_match(version) {
        println!("Invalid version string");
        return Err("Invalid version string".into());
    }

    let all_versions: Vec<String> = get_all_versions(base_node_url)?;
    let version_str: String = "v".to_string() + version;

    if !all_versions.contains(&version_str) {
        println!("Version {} is not available", version);
        return Err("Version not available".into());
    }

    println!("Downloading and installing version {}", version);

    Ok(version.to_string())
}
