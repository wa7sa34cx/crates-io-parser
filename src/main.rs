//! Main module

// https://crates.io/crates?sort=downloads&page=1

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://crates.io/crates?sort=downloads&page=1";
    let resp = reqwest::get(url).await?;
        
    println!("{:#?}", resp);

    Ok(())
}
