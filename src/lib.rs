#[cfg(feature = "async")]
mod r#async;

#[cfg(feature = "async")]
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
pub use r#async::pinned;

#[cfg(feature = "blocking")]
/// Fetches the pinned repositories of a github user in blocking manner.
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
/// use gh_pinned_rs::blocking::pinned;
///
/// fn main() {
///     match pinned("qxb3") {
///         Ok(repos) => println!("Pinned repositories: {:?}", repos),
///         Err(err) => eprintln!("Error: {}", err),
///     }
/// }
/// ```
pub mod blocking;

mod types;
pub use types::*;

mod utils;
