#![no_std]

use aidoku::{
    Chapter, ContentRating, DeepLinkHandler, DeepLinkResult, FilterValue, ImageRequestProvider,
    Listing, ListingProvider, Manga, MangaPageResult, MangaStatus, Page, Result, Source, Viewer,
    alloc::{String, Vec, string::ToString, vec},
    helpers::uri::encode_uri_component,
    imports::net::Request,
    prelude::*,
};

mod home;
mod net;
mod parser;

pub(crate) const BASE_URL: &str = "https://www.4khd.com";

struct FourKhd;

impl Source for FourKhd {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(
        &self,
        query: Option<String>,
        page: i32,
        filters: Vec<FilterValue>,
    ) -> Result<MangaPageResult> {
        let query = query.or_else(|| {
            filters.into_iter().find_map(|filter| match filter {
                FilterValue::Select { id, value }
                    if matches!(id.as_str(), "tag" | "genre") && !value.is_empty() =>
                {
                    Some(value)
                }
                _ => None,
            })
        });
        let base = query
            .as_deref()
            .filter(|query| !query.trim().is_empty())
            .map(|query| format!("{BASE_URL}/?s={}", encode_uri_component(query.trim())))
            .unwrap_or_else(|| format!("{BASE_URL}/"));
        let url = paged_url(&base, page);
        Ok(parser::gallery_list(&net::request(&url)?))
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
            manga.title = parser::title(&html).unwrap_or(manga.title);
            manga.cover = parser::cover(&html).or(manga.cover);
            manga.tags = parser::title_tags(&manga.title);
            manga.url = Some(url.clone());
            manga.status = MangaStatus::Completed;
            manga.content_rating = ContentRating::NSFW;
            manga.viewer = Viewer::RightToLeft;
        }
        if needs_chapters {
            manga.chapters = Some(vec![Chapter {
                key: manga.key.clone(),
                title: Some("Gallery".to_string()),
                chapter_number: Some(1.0),
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
            let html = net::request(&format!("{first_url}/{page}"))?;
            parser::append_images(&html, &mut pages);
        }
        Ok(pages)
    }
}

impl ListingProvider for FourKhd {
    fn get_manga_list(&self, listing: Listing, page: i32) -> Result<MangaPageResult> {
        if listing.id != "/" && listing.id != "/pages/popular" {
            bail!("Invalid listing");
        }
        let base = format!("{BASE_URL}{}", listing.id);
        Ok(parser::gallery_list(&net::request(&paged_url(
            &base, page,
        ))?))
    }
}

impl ImageRequestProvider for FourKhd {
    fn get_image_request(
        &self,
        url: String,
        _context: Option<aidoku::PageContext>,
    ) -> Result<Request> {
        let url = parser::normalize_image_url(url);
        net::image_request(&url)
    }
}

impl DeepLinkHandler for FourKhd {
    fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
        let Some(path) = url.strip_prefix(BASE_URL) else {
            return Ok(None);
        };
        let path = path.split(['?', '#']).next().unwrap_or(path);
        if !path.starts_with("/content/") {
            return Ok(None);
        }
        Ok(Some(DeepLinkResult::Manga {
            key: path.to_string(),
        }))
    }
}

fn paged_url(base: &str, page: i32) -> String {
    if page <= 1 {
        return base.to_string();
    }
    let separator = if base.contains('?') { '&' } else { '?' };
    format!("{base}{separator}query-3-page={page}")
}

register_source!(
    FourKhd,
    Home,
    ListingProvider,
    ImageRequestProvider,
    DeepLinkHandler
);
