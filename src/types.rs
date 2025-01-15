#[derive(Debug)]
pub struct PinnedRepo {
    pub name: String,
    pub description: String,
    pub stars: Option<i32>,
    pub forks: Option<i32>,
    pub language: Option<Language>
}

impl PinnedRepo {
    pub fn new(
        name: String,
        description: String,
        stars: Option<i32>,
        forks: Option<i32>,
        language: Option<Language>
    ) -> Self {
        Self {
            name,
            description,
            stars,
            forks,
            language
        }
    }
}

#[derive(Debug)]
pub struct Language {
    pub color: String,
    pub name: String
}

impl Language {
    pub fn new(color: String, name: String) -> Self {
        Self {
            color,
            name
        }
    }
}

pub type PinnedResult = Result<Vec<PinnedRepo>, String>;
