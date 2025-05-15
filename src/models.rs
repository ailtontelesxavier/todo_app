
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

pub enum TopdoToggleAction {
    
}