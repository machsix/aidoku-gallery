use aidoku::{
    Result,
    imports::{html::Document, net::Request},
};

pub const REFERER: &str = "https://www.4khd.com/";
const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_5 like Mac OS X) \
                          AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 \
                          Mobile/15E148 Safari/604.1";

pub fn request(url: &str) -> Result<Document> {
    Ok(image_request(url)?.html()?)
}

pub fn image_request(url: &str) -> Result<Request> {
    Ok(Request::get(url)?
        .header("Referer", REFERER)
        .header("User-Agent", USER_AGENT))
}
