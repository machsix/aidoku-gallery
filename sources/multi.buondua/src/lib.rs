#![no_std]

use aidoku::{
    Chapter, ContentRating, DeepLinkHandler, DeepLinkResult, FilterValue, ImageRequestProvider,
    Listing, ListingProvider, Manga, MangaPageResult, MangaStatus, Page, Result, Source, Viewer,
    alloc::{String, Vec, string::ToString, vec},
    helpers::uri::QueryParameters,
    imports::{net::Request, std::parse_date},
    prelude::*,
};

mod home;
mod net;
mod parser;

pub(crate) const BASE_URL: &str = "https://buondua.com";

struct Buondua;

impl Source for Buondua {
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
        let html = net::request(&url)?;
        Ok(parser::gallery_list(&html))
    }

    fn get_manga_update(
        &self,
        mut manga: Manga,
        needs_details: bool,
        needs_chapters: bool,
    ) -> Result<Manga> {
        let url = format!("{BASE_URL}{}", manga.key);
        let html = net::request(&url)?;

        if needs_details {
            manga.title = html
                .select_first(".article-header h1")
                .and_then(|element| element.text())
                .map(parser::clean_title)
                .unwrap_or(manga.title);
            manga.cover = html
                .select_first(".article-fulltext img")
                .and_then(|image| image.attr("abs:src"))
                .or(manga.cover);
            manga.tags = html.select(".article-tags a.tag").map(|elements| {
                elements
                    .filter_map(|element| element.text())
                    .map(|tag| tag.trim().to_string())
                    .collect::<Vec<_>>()
            });
            manga.status = MangaStatus::Completed;
            manga.content_rating = ContentRating::NSFW;
            manga.viewer = Viewer::RightToLeft;
            manga.url = Some(url.clone());
        }

        if needs_chapters {
            let date_uploaded = html
                .select_first(".article-info small:last-child")
                .and_then(|element| element.text())
                .and_then(|date| parse_date(date.trim(), "HH:mm dd-MM-yyyy"));
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
        let first_html = net::request(&first_url)?;
        let count = parser::page_count(&first_html);
        let mut pages = Vec::new();
        parser::append_images(&first_html, &mut pages);

        for page in 2..=count {
            let html = net::request(&format!("{first_url}?page={page}"))?;
            parser::append_images(&html, &mut pages);
        }

        Ok(pages)
    }
}

impl ImageRequestProvider for Buondua {
    fn get_image_request(
        &self,
        url: String,
        _context: Option<aidoku::PageContext>,
    ) -> Result<Request> {
        net::image_request(&url)
    }
}

impl ListingProvider for Buondua {
    fn get_manga_list(&self, listing: Listing, page: i32) -> Result<MangaPageResult> {
        let path = listing.id.as_str();
        if path != "/" && !path.starts_with("/tag/") {
            bail!("Invalid listing");
        }

        let mut parameters = QueryParameters::new();
        if page > 1 {
            parameters.push("start", Some(&((page - 1) * 20).to_string()));
        }
        let url = if page > 1 {
            format!("{BASE_URL}{path}?{parameters}")
        } else {
            format!("{BASE_URL}{path}")
        };
        let html = net::request(&url)?;
        Ok(parser::gallery_list(&html))
    }
}

impl DeepLinkHandler for Buondua {
    fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
        let Some(path) = url.strip_prefix(BASE_URL) else {
            return Ok(None);
        };
        let path = path.split(['?', '#']).next().unwrap_or(path);
        let Some((_, id)) = path.rsplit_once('-') else {
            return Ok(None);
        };
        if !path.starts_with('/') || !id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Ok(None);
        }

        Ok(Some(DeepLinkResult::Manga {
            key: path.to_string(),
        }))
    }
}

register_source!(
    Buondua,
    Home,
    ListingProvider,
    ImageRequestProvider,
    DeepLinkHandler
);
