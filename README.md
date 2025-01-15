<h1 align="center">gh-pinned-rs</h1>
<p align="center">
    <img src="https://img.shields.io/crates/v/gh-pinned-rs" />
    <img src="https://img.shields.io/crates/dr/gh-pinned-rs" />
</p>
<p align="center">Fetch pinned repositories from github.</p>

## Installation

```bash
cargo add gh-pinned-rs
```

```toml
[dependencies]
gh-pinned-rs = "2.0.4"
```

## Example

```rust
use gh_pinned_rs::pinned;

#[tokio::main]
async fn main() {
    match pinned("qxb3").await {
        Ok(repos) => println!("Pinned repositories: {:?}", repos),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## Blocking

```bash
cargo add gh-pinned-rs --features blocking
```

```toml
[dependencies]
gh-pinned-rs = { version = "2.0.4", features = ["blocking"] }
```

### Blocking Example

```rust
use gh_pinned_rs::blocking::pinned;

fn main() {
    match pinned("qxb3") {
        Ok(repos) => println!("Pinned repositories: {:?}", repos),
        Err(err) => eprintln!("Error: {}", err),
    }
}
```

## Contribution

Contributions to gh-pinned-rs are welcome! If you have ideas for improvements, new features, or bug fixes, feel free to open an issue or submit a pull request on [gh-pinned-rs](https://github.com/qxb3/gh-pinned-rs)
