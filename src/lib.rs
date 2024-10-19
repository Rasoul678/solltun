mod functions_lib;
pub use functions_lib::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

slint::include_modules!();
