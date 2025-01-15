use crate::{types::PinnedResult, utils};

/// Fetches the pinned repositories of a github user asynchronously.
///
/// # Arguments
///
/// * `name` - The GitHub username whose pinned repositories you want to fetch.
///
/// # Returns
///
/// Returns a `PinnedResult` containing the list of pinned repositories, or an error if the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP request to fetch the GitHub page fails.
/// - The response body cannot be decoded into text.
/// - Parsing the HTML content fails to extract pinned repositories.
///
/// # Example
///
/// ```rust
/// use gh_pinned_rs::pinned;
///
/// #[tokio::main]
/// async fn main() {
///     match pinned("qxb3").await {
///         Ok(repos) => println!("Pinned repositories: {:?}", repos),
///         Err(err) => eprintln!("Error: {}", err),
///     }
/// }
/// ```
pub async fn pinned(name: &str) -> PinnedResult {
    let gh_page = reqwest::get(format!("https://github.com/{name}")).await
        .map_err(|err| format!("Failed to fetch github page: {err}"))?;

    let html = gh_page.text().await
        .map_err(|err| format!("Failed to decode github page: {err}"))?;

    let pinned_repos = utils::get_pinned_repos(html.as_str())?;

    Ok(pinned_repos)
}
