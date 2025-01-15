use tl::NodeHandle;

use crate::{PinnedRepo, PinnedResult};

pub fn get_pinned_repos(html: &str) -> PinnedResult {
    let mut repos: Vec<PinnedRepo> = vec![];

    let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let pinned_repos: Vec<NodeHandle> = dom.get_elements_by_class_name("pinned-item-list-item-content").collect();

    for pinned_html in pinned_repos.iter().map(|&e| e.get(parser).unwrap().inner_html(parser).into_owned()) {
        let pinned_dom = tl::parse(pinned_html.as_str(), tl::ParserOptions::default()).unwrap();
        let pinned_parser = pinned_dom.parser();

        let name_dom = pinned_dom
            .get_elements_by_class_name("repo")
            .collect::<Vec<NodeHandle>>();

        let description_dom = pinned_dom
            .get_elements_by_class_name("pinned-item-desc")
            .collect::<Vec<NodeHandle>>();

        let pinned_meta = pinned_dom
            .get_elements_by_class_name("pinned-item-meta")
            .collect::<Vec<NodeHandle>>();

        // name & description should be present at all times
        let name = name_dom.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().to_string();
        let description = description_dom.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().to_string();

        let stars = match pinned_meta.first() {
            Some(description) => Some(description.get(pinned_parser)
                                            .ok_or("Failed to parse stars dom".to_string())?
                                            .inner_text(pinned_parser)
                                            .trim()
                                            .parse::<i32>()
                                            .map_err(|err| format!("Failed to convert stars element to i32: {err}"))?),
            None => None
        };

        let forks = match pinned_meta.last() {
            Some(description) => Some(description.get(pinned_parser)
                                            .ok_or("Failed to parse forks dom".to_string())?
                                            .inner_text(pinned_parser)
                                            .trim()
                                            .parse::<i32>()
                                            .map_err(|err| format!("Failed to convert forks element to i32: {err}"))?),
            None => None
        };

        repos.push(PinnedRepo::new(
            name,
            description,
            stars,
            forks
        ));
    }

    Ok(repos)
}
