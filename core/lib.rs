use anyhow::Result;
use nipper::Document;
use regex::Regex;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use url::Url;

pub mod comic;
pub mod images;
pub mod yt;

pub use comic::{ehentai, nhentai, pixiv, Comic, Image};
pub use images::instagram;

pub async fn start(url: &str) -> Result<()> {
    yt::main(url).await?;

    // instagram::download().await?;

    Ok(())
}

pub async fn foundry(url: &str) -> Result<Image> {
    let client = ClientBuilder::new().cookie_store(true).build()?;

    let resp = client.get(url).send().await?.text().await?;
    fs::write("temp/foundry.html", &resp).await?;

    let document = Document::from(&resp);
    let img = document.select("img.center").attr("src").unwrap();
    let img: Url = img.replacen("//", "https://", 1).parse()?;

    Ok(Image { url: img })
}

pub async fn download_comic(comic: &Comic) -> Result<()> {
    for page in comic.pages.iter() {
        let path = format!("temp/{}/{}", comic.site, comic.id);
        println!("{}", &path);

        fs::create_dir_all(&path).await?;

        let full_path = format!("{}/{}", &path, page.file_name);
        let full_path = Path::new(&full_path);

        println!("{:#?}", full_path);

        let image = reqwest::get(page.url.to_owned()).await?.bytes().await?;
        fs::write(full_path, image).await?;
    }

    Ok(())
}
