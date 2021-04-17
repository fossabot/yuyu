use anyhow::Result;
use url::Url;

pub mod reader;
use reader::Reader;

use crate::comic::{Comic, Page};

pub async fn get_comic(url: &str) -> Result<Comic> {
    let resp = reqwest::get(url).await?.text().await?;
    let reader = Reader::from_page(&resp)?;

    let cover: Page = {
        let url = format!(
            "https://t.nhentai.net/galleries/{}/cover.{}",
            reader.gallery.media_id, reader.gallery.images.cover.image_type
        )
        .parse()?;

        let file_name = format!(
            "cover.{}",
            reader.gallery.images.cover.image_type.to_string()
        );

        Page {
            url,
            width: Some(reader.gallery.images.cover.width),
            heigth: Some(reader.gallery.images.cover.heigth),
            file_name,
        }
    };

    let mut pages: Vec<Page> = vec![];

    for (index, page) in reader.gallery.images.pages.iter().enumerate() {
        let url: Url = format!(
            "{}galleries/{}/{}.{}",
            reader.media_url,
            reader.gallery.media_id,
            index + 1,
            page.image_type
        )
        .parse()?;

        let file_name = format!("{}.{}", index + 1, page.image_type.to_string());

        pages.push(Page {
            url,
            width: Some(page.width),
            heigth: Some(page.heigth),
            file_name,
        })
    }

    let mut authors: Vec<String> = vec![];
    let mut categories: Vec<String> = vec![];
    let mut characters: Vec<String> = vec![];
    let mut groups: Vec<String> = vec![];
    let mut languages: Vec<String> = vec![];
    let mut tags: Vec<String> = vec![];
    let mut translated: bool = false;

    for tag in reader.gallery.tags {
        let name = tag.name;
        match tag.tag_type {
            reader::TagType::Tag => tags.push(name),
            reader::TagType::Language => {
                if name == "translated" {
                    translated = true
                } else {
                    languages.push(name)
                }
            }
            reader::TagType::Parody => categories.push(name),
            reader::TagType::Character => characters.push(name),
            reader::TagType::Group => groups.push(name),
            reader::TagType::Artist => authors.push(name),
            reader::TagType::Category => categories.push(name),
        }
    }

    Ok(Comic {
        authors,
        categories,
        characters,
        cover,
        id: reader.gallery.id.to_string(),
        groups,
        languages,
        pages,
        site: "nhentai".to_string(),
        tags,
        title: reader.gallery.title.pretty,
        translated,
        upload_date: reader.gallery.upload_date,
    })
}
