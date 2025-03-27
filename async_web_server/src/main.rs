use std::fs::read_to_string;

use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
// this is the content of a website that has html file
    let html = r#"

                <html>
            <body>
                <a href="https://example.com">Example</a>
                <a href="https://rust-lang.org">Rust</a>
            </body>
        </html>

    "#;
// This is to convert the string of text receive to an html file and return an error if the file could not be converted
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap(); // this is the criteria with which the selection is madTe

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href"){
            println!("Found Link: {link}");
        }
    }

let fragment = Html::parse_fragment(r#"<h1 id="new">Hello, <i>world!</i></h1>"#);
let selector = Selector::parse("h1").unwrap();

let h1 = fragment.select(&selector).next().unwrap();

    let value = h1.value().attr("").unwrap();

    


}
