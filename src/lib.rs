use std::{fmt::Display, time::SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeType {
    Pin,
    Fob,
}

impl Display for CodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            CodeType::Pin => "pin",
            CodeType::Fob => "fob",
        };
        write!(f, "{}", code)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Audit {
    pub timestamp: SystemTime,
    pub code: i32,
    pub code_type: CodeType,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserAction {
    Add(i32),
    Del(i32),
    Replace { old: i32, new: i32 },
    Bulk(Vec<i32>),
}
