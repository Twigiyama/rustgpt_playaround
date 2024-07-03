use std::io::{Error, ErrorKind};

async fn my_async_call(url: &str) -> Result<serde_json::Value, Error> {
    let response = reqwest::get(url)
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Failed to fetch data"))?;

    let json_response = response
    .json::<serde_json::Value>()
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Failed to parse JSON"))?;
    
    Ok(json_response)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]

    async fn tests_calls_async_fn() {
        let api_url: &str = "https://catfact.ninja/breeds?limit=1";
        let my_result = my_async_call(api_url).await;
        match my_result {
            Ok(data) => {
                dbg!(data);
            }
            Err(e) => {
                println!("There was an error: {:?}", e);
            }
        }
    }

}

