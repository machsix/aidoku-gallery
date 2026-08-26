use aidoku::{
    Home, HomeComponent, HomeComponentValue, HomeLayout, Link, Listing, Result,
    alloc::{Vec, format, string::ToString},
};

use crate::{FourKhd, net, parser};

impl Home for FourKhd {
    fn get_home(&self) -> Result<HomeLayout> {
        let latest = links(net::REFERER)?;
        let popular = links(&format!("{}/pages/popular", crate::BASE_URL))?;
        let mut components = Vec::new();

        push_scroller(&mut components, "Recently Updated", "/", latest);
        push_scroller(&mut components, "Popular", "/pages/popular", popular);

        Ok(HomeLayout { components })
    }
}

fn links(url: &str) -> Result<Vec<Link>> {
    Ok(parser::gallery_list(&net::request(url)?)
        .entries
        .into_iter()
        .map(Into::into)
        .collect())
}

fn push_scroller(components: &mut Vec<HomeComponent>, title: &str, id: &str, entries: Vec<Link>) {
    if entries.is_empty() {
        return;
    }
    components.push(HomeComponent {
        title: Some(title.to_string()),
        subtitle: None,
        value: HomeComponentValue::Scroller {
            entries,
            listing: Some(Listing {
                id: id.to_string(),
                name: title.to_string(),
                ..Default::default()
            }),
        },
    });
}
