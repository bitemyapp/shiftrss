extern crate clap;
extern crate shiftrss;

use clap::{Arg, App};
use shiftrss::*;

fn parse_item_filter(matches: &clap::ArgMatches) -> ItemFilter {
    let ie = match matches.value_of("exclude").unwrap_or("include") {
        "exclude" => IncludeExclude::Exclude,
        "include" => IncludeExclude::Include,
        _ => IncludeExclude::Include,
    };
    let ifield = match matches.value_of("field").unwrap_or("title") {
        "title" => ItemField::ItemTitle,
        "description" => ItemField::ItemDescription,
        "link" => ItemField::ItemLink,
        _ => ItemField::ItemTitle,
    };
    let ic = match matches.value_of("nocontain").unwrap_or("contains") {
        "nocontain" => ItemContains::ItemDoesNotContain,
        "contains" => ItemContains::ItemDoesContain,
        _ => ItemContains::ItemDoesContain,
    };
    let filter_string = matches.value_of("match").unwrap().to_string();
    ItemFilter {
        include_exclude: ie,
        item_field: ifield,
        item_contains: ic,
        filter_string: filter_string,
    }
}

fn main() {
    let matches = App::new("shiftrss")
        .version("0.0.1")
        .author("Chris Allen <cma@bitemyapp.com>")
        .about("Filtering RSS feeds with simple rules")
        .arg(Arg::with_name("file")
             // .short("file")
             .long("file")
             .value_name("filename")
             .help("File you want to read the RSS feed from")
             .required_unless("url")
             .conflicts_with("url")
             .takes_value(true))
        .arg(Arg::with_name("url")
             .short("u")
             .long("url")
             .value_name("url")
             .help("HTTP URL you want to read the RSS feed from")
             .required_unless("file")
             .conflicts_with("file")
             .takes_value(true))
        .arg(Arg::with_name("exclude")
             .short("e")
             .long("exclude")
             .value_name("exclude")
             .help("Exclude the content that matches the rule provided")
             .required(false)
             .default_value("exclude")
             .conflicts_with("include")
             .takes_value(false))
        .arg(Arg::with_name("include")
             .short("i")
             .long("include")
             .value_name("include")
             .help("Include the content that matches the rule provided")
             .required(false)
             .default_value("include")
             .conflicts_with("exclude")
             .takes_value(false))
        .arg(Arg::with_name("field")
             // .short("field")
             .long("field")
             .value_name("field")
             .help("RSS/Atom item field you want to match on")
             .required(false)
             .default_value("title")
             .possible_value("title")
             .possible_value("description")
             .possible_value("link")
             .takes_value(true))
        .arg(Arg::with_name("contains")
             .short("c")
             .long("contains")
             .value_name("includecontains")
             .help("Contain should contain the value provided")
             .required(false)
             .default_value("contains")
             .conflicts_with("nocontain")
             .takes_value(false))
        .arg(Arg::with_name("nocontain")
             .short("nc")
             .long("nocontain")
             .value_name("nocontain")
             .help("Contain should NOT contain the value provided")
             .required(false)
             .default_value("nocontain")
             .conflicts_with("contains")
             .takes_value(false))
        .arg(Arg::with_name("match")
             .short("m")
             .long("match")
             .value_name("match")
             .help("String you'd like to match against the contents of the RSS")
             .required(true)
             .takes_value(true))
        .get_matches();
    let item_filter = parse_item_filter(&matches);
    let file = matches.value_of("file");
    let url = matches.value_of("url");
    match (file, url) {
        (None, Some(url)) => unimplemented!(),
        (Some(filename), None) => open_rss_file(filename.to_string(), item_filter),
        _ => panic!("At least file or url should be specified!"),
    }
}
