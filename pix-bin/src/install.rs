use reqwest::{get, Error};

pub async fn install() {
    let data = get("https://pix.koompi.org/packages").await;
    match data {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(e) => println!("{}", e),
    }
}
