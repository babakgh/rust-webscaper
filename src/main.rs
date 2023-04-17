/*
Writing a web scaraper in rust and copilot
*/

use reqwest;

fn main() {
    let url = "https://www.rust-lang.org/";
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    // parse the html 
    let document = scraper::Html::parse_document(&body);

    // print the title of the page
    let selector = scraper::Selector::parse("title").unwrap();

    for element in document.select(&selector) {
        println!("{}", element.inner_html());
    }
}