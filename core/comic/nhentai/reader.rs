use anyhow::Result;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct Reader {
    pub media_url: Url,
    pub gallery: Gallery,
    pub start_page: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gallery {
    pub id: u32,
    pub media_id: String,
    pub title: Title,
    pub images: Images,
    pub scanlator: String,
    pub upload_date: f64,
    pub tags: Vec<Tag>,
    pub num_pages: u32,
    pub num_favorites: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Title {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Images {
    pub pages: Vec<Page>,
    pub cover: Page,
    pub thumbnail: Page,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "t")]
    pub image_type: ImageType,
    #[serde(rename = "w")]
    pub width: u32,
    #[serde(rename = "h")]
    pub heigth: u32,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum ImageType {
    #[serde(rename = "j")]
    Jpeg,
    #[serde(rename = "p")]
    Png,
    #[serde(rename = "g")]
    Gif,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: u32,
    #[serde(rename = "type")]
    pub tag_type: TagType,
    pub name: String,
    // could probably be url::Url
    pub url: String,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TagType {
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "language")]
    Language,
    #[serde(rename = "parody")]
    Parody,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "artist")]
    Artist,
    #[serde(rename = "category")]
    Category,
}

impl Reader {
    pub fn from_page(page: &str) -> Result<Reader> {
        let gallery = Gallery::from_page(page)?;

        let media_url_regex = RegexBuilder::new(r#"media_url:\s*'(.*)'"#).build()?;
        let start_page_regex = RegexBuilder::new(r#"start_page:\s*(.*)\s*\}"#).build()?;

        let media_url: Url = media_url_regex
            .captures(page)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()?;

        let start_page: u32 = start_page_regex
            .captures(page)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()?;

        Ok(Reader {
            media_url,
            gallery,
            start_page,
        })
    }
}

impl Gallery {
    pub fn from_page(page: &str) -> Result<Self> {
        let gallery_regex = RegexBuilder::new(r#"window\._gallery\s*=\s*JSON\.parse\("(.*)"\)"#)
            .dot_matches_new_line(true)
            .build()?;

        let gallery = gallery_regex
            .captures(page)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .replace(r"\u0022", r#"""#);

        let gallery: Gallery = serde_json::from_str(&gallery)?;

        Ok(gallery)
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ImageType::Jpeg => {
                write!(f, "jpg")
            }
            ImageType::Png => {
                write!(f, "png")
            }
            ImageType::Gif => {
                write!(f, "gif")
            }
        }
    }
}
