use aidoku::{
    ContentRating, Manga, MangaPageResult, MangaStatus, Page, PageContent, Viewer,
    alloc::{String, Vec, string::ToString},
    imports::html::Document,
};

use crate::BASE_URL;

pub fn gallery_list(html: &Document) -> MangaPageResult {
    let entries = html
        .select("ul.wp-block-post-template > li.wp-block-post")
        .map(|elements| {
            elements
                .filter_map(|element| {
                    let link = element.select_first("h2.wp-block-post-title a")?;
                    let url = link.attr("abs:href")?;
                    let key = url.strip_prefix(BASE_URL)?.to_string();
                    let title = link.text()?.trim().to_string();
                    if title.is_empty() || !key.starts_with("/content/") {
                        return None;
                    }
                    let cover = element
                        .select_first("figure.wp-block-post-featured-image img")
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

    let current_page = html
        .select_first("nav.wp-block-query-pagination .page-numbers.current")
        .and_then(|element| element.text())
        .and_then(|text| text.trim().replace(',', "").parse::<i32>().ok())
        .unwrap_or(1);
    let has_next_page = html
        .select("nav.wp-block-query-pagination a.page-numbers")
        .map(|mut links| {
            links.any(|link| {
                link.text()
                    .map(|text| text.trim().replace(',', ""))
                    .and_then(|text| text.parse::<i32>().ok())
                    .is_some_and(|page| page > current_page)
            })
        })
        .unwrap_or(false);

    MangaPageResult {
        entries,
        has_next_page,
    }
}

pub fn title(html: &Document) -> Option<String> {
    html.select_first("main .wp-block-post-title")
        .and_then(|element| element.text())
        .map(|title| title.trim().to_string())
        .filter(|title| !title.is_empty())
}

pub fn cover(html: &Document) -> Option<String> {
    html.select_first("meta[property='og:image']")
        .and_then(|element| element.attr("content"))
        .or_else(|| {
            html.select_first("meta[name='twitter:image']")
                .and_then(|element| element.attr("content"))
        })
        .or_else(|| {
            html.select_first("main .wp-block-post-content img")
                .and_then(|element| element.attr("abs:src"))
        })
}

pub fn title_tags(title: &str) -> Option<Vec<String>> {
    let mut tags = Vec::new();

    for token in title.split(|character: char| !character.is_alphanumeric()) {
        if token.is_empty() || token.chars().all(|character| character.is_ascii_digit()) {
            continue;
        }
        if !tags
            .iter()
            .any(|tag: &String| tag.eq_ignore_ascii_case(token))
        {
            tags.push(token.to_string());
        }
    }

    (!tags.is_empty()).then_some(tags)
}

pub fn page_count(html: &Document) -> i32 {
    html.select(".page-link-box a.page-numbers")
        .map(|links| {
            links
                .filter_map(|link| link.text()?.trim().parse::<i32>().ok())
                .max()
                .unwrap_or(1)
        })
        .unwrap_or(1)
}

pub fn append_images(html: &Document, pages: &mut Vec<Page>) {
    if let Some(images) = html.select("main .wp-block-post-content img") {
        pages.extend(images.filter_map(|image| {
            let url = image.attr("abs:src")?;
            Some(Page {
                content: PageContent::url(url),
                ..Default::default()
            })
        }));
    }
}
