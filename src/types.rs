#[derive(Debug)]
pub struct PinnedRepo {
    pub name: String,
    pub description: String,
    pub stars: Option<i32>,
    pub forks: Option<i32>
}

impl PinnedRepo {
    pub fn new(name: String, description: String, stars: Option<i32>, forks: Option<i32>) -> Self {
        Self {
            name,
            description,
            stars,
            forks
        }
    }
}

pub type PinnedResult = Result<Vec<PinnedRepo>, String>;
