use std::{env, fs::{self, create_dir_all}, io::Cursor};



#[tokio::main]
async fn main() {
    //collecting arguments
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).unwrap();
    
    let response = reqwest::get(url.as_str()).await.unwrap();
    create_dir_all("downloads").expect("Failed to create dir");
    let file_name = args.get(2).unwrap();
    let file_path = format!("downloads/{}",file_name);
    // creating a dir to store downloaded files
    let mut file_name = fs::File::create(&file_path).expect("Failed to create file");

    // after creating the file it is time to put the content of the downloaded file into it

    let mut content = Cursor::new(response.bytes().await.expect("Failed to get the contents of the downloaded file"));
    std::io::copy(&mut content, &mut file_name).expect("Failed to write content to file");

    println!("File Downloaded Successfully to {} ", file_path);

}
