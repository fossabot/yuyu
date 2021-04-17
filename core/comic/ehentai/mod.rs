use anyhow::Result;
use nipper::Document;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::vec;
use url::Url;

use crate::comic::{Comic, Page};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRequest {
    pub method: String,
    pub gidlist: Vec<(i32, String)>,
    pub namespace: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub gmetadata: Vec<GMetaData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GMetaData {
    pub gid: u32,
    pub token: String,
    pub archiver_key: String,
    pub title: String,
    pub title_jpn: String,
    pub category: String,
    pub thumb: Url,
    pub uploader: String,
    pub posted: String,
    pub filecount: String,
    pub filesize: u32,
    pub expunged: bool,
    pub rating: String,
    #[serde(rename = "torrentcount")]
    pub torrent_count: String,
    pub torrents: Vec<serde_json::Value>,
    pub tags: Vec<String>,
}

pub async fn get_comic(url: &str) -> Result<Comic> {
    let metadata = get_metadata(url).await?;
    let resp = reqwest::get(url).await?.text().await?;
    let document = Document::from(&resp);

    let mut authors: Vec<String> = vec![];
    let mut categories: Vec<String> = vec![];
    let mut characters: Vec<String> = vec![];
    let mut groups: Vec<String> = vec![];
    let mut languages: Vec<String> = vec![];
    let mut tags: Vec<String> = vec![];
    let mut translated = false;

    for tag in metadata.tags {
        match tag.split_once(':') {
            None => tags.push(tag),
            Some((namespace, tag)) => {
                match namespace {
                    "artist" => authors.push(tag.to_string()),
                    "character" => characters.push(tag.to_string()),
                    "female" => tags.push(tag.to_string()),
                    "group" => groups.push(tag.to_string()),
                    "language" => {
                        if tag == "translated" {
                            translated = true
                        } else {
                            languages.push(tag.to_string())
                        }
                    }
                    "male" => tags.push(tag.to_string()),
                    "parody" => categories.push(tag.to_string()),
                    "reclass" => categories.push(tag.to_string()),
                    _ => (), // TODO: Handle invalid namespace
                }
            }
        }
    }

    let cover = {
        let url = metadata.thumb;
        let file_name = url.path_segments().unwrap().last().unwrap().to_string();

        Page {
            file_name,
            heigth: None,
            url,
            width: None,
        }
    };

    let mut pages: Vec<Page> = vec![];

    for s in document.select("#gdt > .gdtm > div > a").iter() {
        let href = &s.attr("href").unwrap().to_string();
        let resp = reqwest::get(href).await?.text().await?;
        let url: Url = Document::from(&resp)
            .select("#img")
            .attr("src")
            .unwrap()
            .parse()?;
        let file_name = url.path_segments().unwrap().last().unwrap().to_string();
        // println!("{}", &file_name);

        pages.push(Page {
            file_name,
            heigth: None,
            url,
            width: None,
        })
    }

    Ok(Comic {
        title: metadata.title,
        authors,
        tags,
        upload_date: metadata.posted.parse().unwrap(),
        languages,
        pages,
        cover,
        characters,
        site: "e-hentai".to_string(),
        id: metadata.gid.to_string(),
        categories,
        translated,
        groups,
    })
}

pub async fn get_metadata(url: &str) -> Result<GMetaData> {
    let url_regex = RegexBuilder::new(r"https://e-hentai.org/g/([0-9]+)/([a-zA-Z0-9]+)").build()?;
    let c = url_regex.captures(url).unwrap();

    let gallery_id: i32 = c.get(1).unwrap().as_str().parse()?;
    let gallery_token = c.get(2).unwrap().as_str();

    let client = reqwest::Client::new();
    let res: ApiResponse = client
        .post("https://api.e-hentai.org/api.php")
        .json(&ApiRequest {
            method: "gdata".to_string(),
            gidlist: vec![(gallery_id, gallery_token.to_string())],
            namespace: 1,
        })
        .send()
        .await?
        .json()
        .await?;

    // If the first element does not exist we should handle this in a better way
    let metadata = res.gmetadata.into_iter().nth(0).unwrap();

    Ok(metadata)
}
