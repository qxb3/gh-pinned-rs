use reqwest::Client;
use tl::NodeHandle;
use tokio;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PinnedRepo {
    name: String,
    description: String,
    stars: u32,
    forks: u32
}

impl PinnedRepo {
    pub fn new(name: String, description: String, stars: u32, forks: u32) -> Self {
        Self {
            name,
            description,
            stars,
            forks
        }
    }
}

pub async fn pinned(name: &str) -> Result<Vec<PinnedRepo>, String> {
    let client = Client::new();
    let mut repos: Vec<PinnedRepo> = vec![];

    let gh_page = client.get(name).send().await.map_err(|err| err.to_string())?;
    let html = gh_page.text().await.unwrap();
    let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).unwrap();
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

        let name = name_dom.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().to_string();
        let description = description_dom.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().to_string();
        let stars = pinned_meta.first().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().parse::<u32>().unwrap();
        let forks = pinned_meta.last().unwrap().get(pinned_parser).unwrap().inner_text(pinned_parser).trim().parse::<u32>().unwrap();

        let repo = PinnedRepo::new(
            name,
            description,
            stars,
            forks,
        );

        repos.push(repo);
    }

    Ok(repos)
}
