#![no_std]

use aidoku::{
    Chapter, ContentRating, DeepLinkHandler, DeepLinkResult, FilterValue, ImageRequestProvider,
    Manga, MangaPageResult, MangaStatus, Page, PageContent, Result, Source, Viewer,
    alloc::{String, Vec, string::ToString, vec},
    helpers::uri::QueryParameters,
    imports::{html::Document, net::Request, std::parse_date},
    prelude::*,
};

const BASE_URL: &str = "https://xiutaku.com";
const REFERER: &str = "https://xiutaku.com/";
const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_5 like Mac OS X) \
                          AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 \
                          Mobile/15E148 Safari/604.1";

struct Xiutaku;

impl Xiutaku {
    fn request(url: &str) -> Result<Document> {
        Ok(Request::get(url)?
            .header("Referer", REFERER)
            .header("User-Agent", USER_AGENT)
            .html()?)
    }
}

fn parse_gallery_list(html: &Document) -> MangaPageResult {
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
                        viewer: Viewer::Webtoon,
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

fn page_count(html: &Document) -> i32 {
    let mut count = 1;
    if let Some(links) = html.select(".article nav.pagination a.pagination-link") {
        for link in links {
            if let Some(value) = link.text().and_then(|value| value.trim().parse().ok()) {
                count = count.max(value);
            }
        }
    }
    count
}

fn append_images(html: &Document, pages: &mut Vec<Page>) {
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

impl Source for Xiutaku {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(
        &self,
        query: Option<String>,
        page: i32,
        _filters: Vec<FilterValue>,
    ) -> Result<MangaPageResult> {
        let mut parameters = QueryParameters::new();
        let mut has_parameters = false;
        if let Some(query) = query.as_deref() {
            parameters.push("search", Some(query));
            has_parameters = true;
        }
        if page > 1 {
            parameters.push("start", Some(&((page - 1) * 20).to_string()));
            has_parameters = true;
        }

        let url = if has_parameters {
            format!("{BASE_URL}/?{parameters}")
        } else {
            format!("{BASE_URL}/")
        };
        let html = Self::request(&url)?;
        Ok(parse_gallery_list(&html))
    }

    fn get_manga_update(
        &self,
        mut manga: Manga,
        needs_details: bool,
        needs_chapters: bool,
    ) -> Result<Manga> {
        let url = format!("{BASE_URL}{}", manga.key);
        let html = Self::request(&url)?;

        if needs_details {
            manga.title = html
                .select_first(".article-header h1")
                .and_then(|element| element.text())
                .map(|title| title.trim().to_string())
                .unwrap_or(manga.title);
            manga.cover = html
                .select_first(".article-fulltext img")
                .and_then(|image| image.attr("abs:src"))
                .or(manga.cover);
            manga.description = html
                .select(".article > .article-info")
                .and_then(|mut elements| elements.nth(1))
                .and_then(|element| element.text())
                .map(|description| description.trim().to_string());
            manga.tags = html.select(".article-tags a.tag").map(|elements| {
                elements
                    .filter_map(|element| element.text())
                    .map(|tag| tag.trim().trim_start_matches('#').to_string())
                    .collect::<Vec<_>>()
            });
            manga.status = MangaStatus::Completed;
            manga.content_rating = ContentRating::NSFW;
            manga.viewer = Viewer::Webtoon;
            manga.url = Some(url.clone());
        }

        if needs_chapters {
            let date_uploaded = html
                .select_first(".article > .article-info small")
                .and_then(|element| element.text())
                .map(|date| date.trim().trim_start_matches('🕒').trim().to_string())
                .and_then(|date| parse_date(&date, "HH:mm dd-MM-yyyy"));
            manga.chapters = Some(vec![Chapter {
                key: manga.key.clone(),
                title: Some("Gallery".to_string()),
                chapter_number: Some(1.0),
                date_uploaded,
                url: Some(url),
                ..Default::default()
            }]);
        }

        Ok(manga)
    }

    fn get_page_list(&self, _manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
        let first_url = format!("{BASE_URL}{}", chapter.key);
        let first_html = Self::request(&first_url)?;
        let count = page_count(&first_html);
        let mut pages = Vec::new();
        append_images(&first_html, &mut pages);

        for page in 2..=count {
            let html = Self::request(&format!("{first_url}?page={page}"))?;
            append_images(&html, &mut pages);
        }

        Ok(pages)
    }
}

impl ImageRequestProvider for Xiutaku {
    fn get_image_request(
        &self,
        url: String,
        _context: Option<aidoku::PageContext>,
    ) -> Result<Request> {
        Ok(Request::get(url)?
            .header("Referer", REFERER)
            .header("User-Agent", USER_AGENT))
    }
}

impl DeepLinkHandler for Xiutaku {
    fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
        let Some(path) = url.strip_prefix(BASE_URL) else {
            return Ok(None);
        };
        let path = path.split(['?', '#']).next().unwrap_or(path);
        let Some(id) = path.strip_prefix('/') else {
            return Ok(None);
        };
        if id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Ok(None);
        }

        Ok(Some(DeepLinkResult::Manga {
            key: path.to_string(),
        }))
    }
}

register_source!(Xiutaku, ImageRequestProvider, DeepLinkHandler);
