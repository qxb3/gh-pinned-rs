use crate::{types::PinnedResult, utils};

pub async fn pinned(name: &str) -> PinnedResult {
    let gh_page = reqwest::get(format!("https://github.com/{name}")).await
        .map_err(|err| format!("Failed to fetch github page: {err}"))?;

    let html = gh_page.text().await
        .map_err(|err| format!("Failed to decode github page: {err}"))?;

    let pinned_repos = utils::get_pinned_repos(html.as_str())?;

    Ok(pinned_repos)
}
