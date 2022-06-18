use file::*;

#[tokio::main]
async fn test(){
    let resp = reqwest::get("http://www.google.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    let index = file("index.html");
    index.write(resp.as_str());
}

fn main(){
    test();
}
