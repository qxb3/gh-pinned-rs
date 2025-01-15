#[derive(Debug)]
pub struct PinnedRepo {
    pub name: String,
    pub description: String,
    pub stars: u32,
    pub forks: u32
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
