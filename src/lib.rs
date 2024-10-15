use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}
