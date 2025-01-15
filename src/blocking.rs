use reqwest::blocking;

use crate::{types::PinnedResult, utils};

pub fn pinned(name: &str) -> PinnedResult {
    let gh_page = blocking::get(format!("https://github.com/{name}"))
        .map_err(|err| format!("Failed to fetch github page: {err}"))?;

    let html = gh_page.text()
        .map_err(|err| format!("Failed to decode github page: {err}"))?;

    let pinned_repos = utils::get_pinned_repos(html.as_str())?;

    Ok(pinned_repos)
}
