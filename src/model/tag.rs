use sqlx::FromRow;
use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

//TODO Add tests
