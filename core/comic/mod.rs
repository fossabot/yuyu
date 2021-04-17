use url::Url;

pub mod ehentai;
pub mod nhentai;
pub mod pixiv;

/// A struct containing vital information about a comic.
#[derive(Debug)]
pub struct Comic {
    /// The authors of the comic
    pub authors: Vec<String>,
    pub categories: Vec<String>,
    /// The characters appearing in the comic.
    pub characters: Vec<String>,
    pub cover: Page,
    /// The unique identifier of the comic.
    ///
    /// This is guaranteed to be unique only on the original site,
    /// as such it should always be used in conjunction with the site property.
    pub id: String,
    pub groups: Vec<String>,
    // TODO: Explain why it's a vec in detail
    /// The language this comic is avaible.
    ///
    /// This is usually one but it is a vec for api reason.
    pub languages: Vec<String>,
    pub pages: Vec<Page>,
    /// The site this comic was downloaded from
    pub site: String,
    pub tags: Vec<String>,
    pub title: String,
    /// Whether or not the comic was translated from another language.
    pub translated: bool,
    /// Upload date in UNIX time-stamp format.
    pub upload_date: f64,
}

#[derive(Debug)]
pub struct Page {
    pub file_name: String,
    pub heigth: Option<u32>,
    pub url: Url,
    pub width: Option<u32>,
}

#[derive(Debug)]
pub struct Image {
    pub url: Url,
}
