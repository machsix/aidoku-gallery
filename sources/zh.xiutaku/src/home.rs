use aidoku::{
    Home, HomeComponent, HomeComponentValue, HomeLayout, Link, Listing, Result,
    alloc::{Vec, format, string::ToString},
};

use crate::{BASE_URL, REFERER, Xiutaku, parse_gallery_list};

impl Home for Xiutaku {
    fn get_home(&self) -> Result<HomeLayout> {
        let latest = links(REFERER)?;
        let hotest = links(&format!("{BASE_URL}/hot"))?;
        let mut components = Vec::new();

        push_scroller(&mut components, "Latest", "/", latest);
        push_scroller(&mut components, "Hotest", "/hot", hotest);

        Ok(HomeLayout { components })
    }
}

fn links(url: &str) -> Result<Vec<Link>> {
    Ok(parse_gallery_list(&Xiutaku::request(url)?)
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
