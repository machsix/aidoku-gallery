use aidoku::{
    Home, HomeComponent, HomeComponentValue, HomeLayout, Link, Listing, Result,
    alloc::{Vec, format, string::ToString},
};

use crate::{Buondua, net, parser};

impl Home for Buondua {
    fn get_home(&self) -> Result<HomeLayout> {
        let latest_html = net::request(net::REFERER)?;
        let latest: Vec<Link> = parser::gallery_list(&latest_html)
            .entries
            .into_iter()
            .map(Into::into)
            .collect();
        let collection_html = net::request(&format!("{}/collection", crate::BASE_URL))?;
        let collections = parser::collection_links(&collection_html);

        let mut components = Vec::new();
        if let Some(component) = scroller_component(
            "Latest Galleries",
            latest,
            Some(Listing {
                id: "/".to_string(),
                name: "Latest Galleries".to_string(),
                ..Default::default()
            }),
        ) {
            components.push(component);
        }
        if let Some(component) = scroller_component("Collections", collections, None) {
            components.push(component);
        }

        Ok(HomeLayout { components })
    }
}

fn scroller_component(
    title: &str,
    entries: Vec<Link>,
    listing: Option<Listing>,
) -> Option<HomeComponent> {
    (!entries.is_empty()).then_some(HomeComponent {
        title: Some(title.to_string()),
        subtitle: None,
        value: HomeComponentValue::Scroller { entries, listing },
    })
}
