use anyhow::Result;
use nipper::Document;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use tokio::fs;
use url::Url;

pub async fn get_comic(url: &str) -> Result<()> {
    let resp = reqwest::get(url).await?.text().await?;
    fs::write("temp/pixi.html", &resp).await?;

    let document = Document::from(&resp);
    let data = document
        .select("#meta-preload-data")
        .attr("content")
        .unwrap()
        .to_string();

    fs::write("temp/p.c.json", &data).await?;

    let config: PixivConfig = serde_json::from_str(&data)?;
    fs::write("temp/config.p.json", serde_json::to_string_pretty(&config)?).await?;

    println!("{:#?}", &config);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PixivConfig {
    pub timestamp: String,
    pub illust: HashMap<String, Illust>,
    pub user: HashMap<String, User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Illust {
    #[serde(rename = "illustId")]
    pub illust_id: String,
    #[serde(rename = "illustTitle")]
    pub illust_title: String,
    #[serde(rename = "illustComment")]
    pub illust_comment: String,
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "illustType")]
    pub illust_type: i32,
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "uploadDate")]
    pub upload_date: String,
    pub restrict: i32,
    #[serde(rename = "xRestrict")]
    pub x_restrict: i32,
    pub sl: i32,
    pub urls: Urls,
    pub tags: Tags,
    pub alt: String,
    #[serde(rename = "storableTags")]
    pub storable_tags: Vec<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userAccount")]
    pub user_account: String,
    pub userIllusts: HashMap<String, Option<UserIllust>>,
    #[serde(rename = "likeData")]
    pub like_data: bool,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "pageCount")]
    pub page_count: u32,
    pub bookmarkCount: u32,
    pub likeCount: u32,
    pub commentCount: u32,
    pub responseCount: u32,
    pub viewCount: u32,
    pub isHowto: bool,
    pub isOriginal: bool,
    pub imageResponseOutData: Vec<serde_json::Value>,
    pub imageResponseData: Vec<serde_json::Value>,
    pub imageResponseCount: u32,
    pub pollData: Option<serde_json::Value>,
    pub seriesNavData: Option<SeriesNavData>,
    pub descriptionBoothId: Option<serde_json::Value>,
    pub descriptionYoutubeId: Option<serde_json::Value>,
    pub comicPromotion: Option<serde_json::Value>,
    // pub fanboxPromotion
    pub contestBanners: Vec<serde_json::Value>,
    pub isBookmarkable: bool,
    pub bookmarkData: Option<serde_json::Value>,
    pub contestData: Option<serde_json::Value>,
    // pub zoneConfig:
    // pub extraData
    // pub titleCaptionTranslation
    pub isUnlisted: bool,
    pub request: Option<serde_json::Value>, // pub noLoginData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Urls {
    pub mini: Url,
    pub thumb: Url,
    pub small: Url,
    pub regular: Url,
    pub original: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub tags: Vec<Tag>,
    pub writable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub locked: bool,
    pub deletable: bool,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIllust {
    pub id: String,
    pub title: String,
    pub illustType: i32,
    pub xRestrict: i32,
    pub restrict: i32,
    pub sl: i32,
    pub url: Url,
    pub description: String,
    pub tags: Vec<String>,
    pub userId: String,
    pub userName: String,
    pub width: u32,
    pub height: u32,
    pub pageCount: u32,
    pub isBookmarkable: bool,
    #[serde(rename = "bookmarkData")]
    pub bookmark_data: Option<serde_json::Value>,
    pub alt: String,
    // pub titleCaptionTranslation
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "updateDate")]
    pub update_date: String,
    #[serde(rename = "isUnlisted")]
    pub is_unlisted: bool,
    #[serde(rename = "isMasked")]
    pub is_masked: bool,
    #[serde(rename = "profileImageUrl")]
    pub profile_image_url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesNavData {
    pub seriesType: String,
    pub seriesId: String,
    pub title: String,
    pub order: u32,
    pub isWatched: bool,
    pub prev: Prev,
    pub next: Next,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Prev {
    pub id: String,
    pub title: String,
    pub order: u32,
}

pub type Next = Prev;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub name: String,
    pub image: Url,
    #[serde(rename = "imageBig")]
    pub image_big: Url,
    pub premium: bool,
    #[serde(rename = "isFollowed")]
    pub is_followed: bool,
    #[serde(rename = "isMypixiv")]
    pub is_mypixiv: bool,
    #[serde(rename = "isBlocking")]
    pub is_blocking: bool,
    pub background: Background,
    #[serde(rename = "sketchLiveId")]
    pub sketch_live_id: Option<serde_json::Value>,
    pub partial: i32,
    #[serde(rename = "acceptRequest")]
    pub accept_request: bool,
    #[serde(rename = "sketchLives")]
    pub sketch_lives: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    pub repeat: Option<serde_json::Value>,
    pub color: Option<serde_json::Value>,
    pub url: Url,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
}
