use tl::NodeHandle;

use crate::{Language, PinnedRepo, PinnedResult};

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

        let language_color_dom = pinned_dom
            .get_elements_by_class_name("repo-language-color")
            .collect::<Vec<NodeHandle>>();

        let language_name_dom = pinned_dom
            .query_selector("span[itemprop=\"programmingLanguage\"]");

        // name & description should be present at all times
        let name = name_dom.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().to_string();
        let description = description_dom.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().to_string();

        let stars = match pinned_meta.first() {
            Some(stars) => Some(stars.get(pinned_parser)
                                            .ok_or("Failed to parse stars dom".to_string())?
                                            .inner_text(pinned_parser)
                                            .trim()
                                            .parse::<i32>()
                                            .map_err(|err| format!("Failed to convert stars element to i32: {err}"))?),
            None => None
        };

        let forks = match pinned_meta.last() {
            Some(forks) => Some(forks.get(pinned_parser)
                                        .ok_or("Failed to parse forks dom".to_string())?
                                        .inner_text(pinned_parser)
                                        .trim()
                                        .parse::<i32>()
                                        .map_err(|err| format!("Failed to convert forks element to i32: {err}"))?),
            None => None
        };

        let language_color = match language_color_dom.first() {
            Some(language_color) => Some(language_color.get(pinned_parser)
                                                            .ok_or("Failed to parse language color dom".to_string())?
                                                            .as_tag()
                                                            .unwrap()
                                                            .attributes()
                                                            .get("style")
                                                            .unwrap()
                                                            .unwrap()
                                                            .as_utf8_str()
                                                            .replace("background-color: ", "")),
            None => None
        };

        let language_name = match language_name_dom {
            Some(language_name) => Some(language_name.collect::<Vec<NodeHandle>>()
                                                        .first()
                                                        .unwrap()
                                                        .get(pinned_parser)
                                                        .ok_or("Failed to parse language name dom".to_string())?
                                                        .inner_text(pinned_parser)
                                                        .trim()
                                                        .to_string()),
            None => None
        };

        let language = match (language_color, language_name) {
            (Some(color), Some(name)) => Some(Language::new(color, name)),
            _ => None
        };

        repos.push(PinnedRepo::new(
            name,
            description,
            stars,
            forks,
            language
        ));
    }

    Ok(repos)
}
