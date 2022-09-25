use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
pub struct Tag {
    id: u32,
    name: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tag {}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Category {
    tag: Tag,
}

impl From<Tag> for Category {
    fn from(tag: Tag) -> Self {
        Self { tag }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.tag))
    }
}