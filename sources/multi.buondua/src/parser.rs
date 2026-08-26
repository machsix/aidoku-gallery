use aidoku::{
    ContentRating, Link, LinkValue, Listing, Manga, MangaPageResult, MangaStatus, Page,
    PageContent, Viewer,
    alloc::{String, Vec, string::ToString},
    imports::html::Document,
};

use crate::BASE_URL;

pub fn clean_title(title: String) -> String {
    match title.find(" - ( Page ") {
        Some(index) => title[..index].trim().to_string(),
        None => title.trim().to_string(),
    }
}

pub fn gallery_list(html: &Document) -> MangaPageResult {
    let entries = html
        .select(".blog > .items-row")
        .map(|elements| {
            elements
                .filter_map(|element| {
                    let link = element.select_first("h2 > a.item-link")?;
                    let url = link.attr("abs:href")?;
                    let key = url.strip_prefix(BASE_URL)?.to_string();
                    let title = link.text()?.trim().to_string();
                    let cover = element
                        .select_first(".item-thumb img")
                        .and_then(|image| image.attr("abs:src"));

                    Some(Manga {
                        key,
                        title,
                        cover,
                        url: Some(url),
                        status: MangaStatus::Completed,
                        content_rating: ContentRating::NSFW,
                        viewer: Viewer::RightToLeft,
                        ..Default::default()
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let has_next_page = html
        .select_first("nav.pagination a[next]")
        .and_then(|element| element.attr("href"))
        .map(|href| href != "javascript:;")
        .unwrap_or(false);

    MangaPageResult {
        entries,
        has_next_page,
    }
}

pub fn collection_links(html: &Document) -> Vec<Link> {
    html.select(".collection > .collection-item")
        .map(|elements| {
            elements
                .filter_map(|element| {
                    let anchor = element.select_first("a.item-link")?;
                    let url = anchor.attr("abs:href")?;
                    let path = url.strip_prefix(BASE_URL)?;
                    let title = anchor.text()?.trim().to_string();
                    if title.is_empty() || !path.starts_with("/tag/") {
                        return None;
                    }
                    let image_url = element
                        .select_first(".thumb-img img")
                        .and_then(|image| image.attr("abs:src"));
                    Some(Link {
                        title: title.clone(),
                        subtitle: None,
                        image_url,
                        value: Some(LinkValue::Listing(Listing {
                            id: path.to_string(),
                            name: title,
                            ..Default::default()
                        })),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn page_count(html: &Document) -> i32 {
    let mut count = 1;
    if let Some(links) = html.select(".article nav.pagination a[href*='?page=']") {
        for link in links {
            if let Some(href) = link.attr("href")
                && let Some((_, value)) = href.split_once("?page=")
                && let Some(value) = value.split('&').next()
                && let Ok(value) = value.parse::<i32>()
            {
                count = count.max(value);
            }
        }
    }
    count
}

pub fn append_images(html: &Document, pages: &mut Vec<Page>) {
    if let Some(images) = html.select(".article-fulltext img") {
        pages.extend(images.filter_map(|image| {
            let url = image.attr("abs:src")?;
            Some(Page {
                content: PageContent::url(url),
                ..Default::default()
            })
        }));
    }
}
