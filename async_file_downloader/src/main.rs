use reqwest;
#[tokio::main]
async fn main() {
    let response = reqwest::get("https://www.cloudflare.com/plans/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{response:?}")
}
