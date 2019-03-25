#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate url;

use regex::Regex;
use scraper::{Html, Selector};
use std::io::prelude::*;
use std::{env, process};

lazy_static! {
    static ref RE: Regex = Regex::new(r".").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();
    if args.len() < 3 {
        writeln!(
            &mut stderr,
            "\nUsage: skimr [website] [selectors 1..n] (Example: skimr nsa.gov tr td.cell div#secret"
        )
        .expect("couldn't write to stderr");
        process::exit(1);
    }
    let mut url: String = String::new();
    if !Regex::new(r"^(http|https)://").unwrap().is_match(&args[1]) {
        url.push_str("http://");
    }
    url.push_str(&args[1]);

    let selectors = &args[2..];
    let mut resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());
    let html = Html::parse_document(&resp.text().unwrap());
    let body = html
        .select(&Selector::parse("html").unwrap())
        .next()
        .unwrap();
    scrape(body, selectors);
}

fn scrape(fragment: scraper::element_ref::ElementRef, selectors: &[String]) {
    if selectors.len() == 0 {
        let output = fragment
            .text()
            .filter(|&i| RE.is_match(i))
            .collect::<Vec<_>>();
        for out in output {
            println!("{}", out);
        }
    } else {
        for node in fragment.select(&Selector::parse(&selectors[0]).unwrap()) {
            scrape(node, &selectors[1..]);
        }
    }
}
