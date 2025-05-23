use std::error;

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // this is the url of the website we are fetching the information from
    let url = "https://www.cloudflare.com/plans/";
    let response = reqwest::get(url).await?; //this  gets the api response from the web server with the all information that is to say it gets the html content from the website
    let body = response.text().await?; // this is to extract the body of the response
    let html = Html::parse_document(&body);

    let selector = Selector::parse("img").expect("Failed to parse selector");

    for element in html.select(&selector) {
        if let Some(val) = element.attr("src") {
            //this one checks if the attribute href is also present in the link is found
            println!("{val}")
        }
    }

    // println!("{html:?}");
    Ok(())
}
