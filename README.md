<h1 align="center">gh-pinned-rs</h1>
<p align="center">
    <img src="https://img.shields.io/crates/v/gh-pinned-rs" />
    <img src="https://img.shields.io/crates/dr/gh-pinned-rs" />
</p>
<p align="center">Fetch pinned repositories from github.</p>

## Installation

```toml
[dependencies]
gh-pinned-rs = "1.0.3"
```

## Example

```rust
use gh_pinned_rs::pinned;

#[tokio::main]
async fn main() -> Result<(), String> {
    let pinned_repos = pinned("qxb3").await?;

    println("{:#?}", pinned_repos);
}
```

## Contribution

Contributions to gh-pinned-rs are welcome! If you have ideas for improvements, new features, or bug fixes, feel free to open an issue or submit a pull request on [gh-pinned-rs](https://github.com/qxb3/gh-pinned-rs)
