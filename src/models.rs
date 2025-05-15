use serde::{Serialize, Deserialize};
use std::{ftm, time::SystemTime};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub is_completed: bool,
    pub created_at: SystemTime,
    pub text: String,
    pub ind: Uuid,
}

impl todo {
    pub fn new (text: &str) -> Self {
        self {
            is_completed: false,
            created_at: SystemTime::now(),
            text: String::from(text),
            ind: Uuid::new_v4(),
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoListFilter {
    Completed,
    Active,
    All,
}

impl fmt::Display for TodoListFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            self::Completed => write!(f, "completed"),
            self::Active => write!(f, "active"),
            self::All => write!(f, "all"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TopdoToggleAction {
    Uncheck,
    Check,
}

impl fmt::Display for TopdoToggleAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            self::Uncheck => write!(f, "uncheck"),
            self::Check => write!(f, "check"),
        }
    }
}