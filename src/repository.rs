

pub enum TodoRepoError {
    NotFound,
}

pub struct TodoRepo {
    pub num_completed: u32,
    pub num_active: u32,
    pub num_all_itens: u32,
    items: HashMap<Uuid, Todo>,
}

impl TodoRepo {
    pub fn get(&self, id: &Uuid) -> Result<Todo, TodoRepoError> {
        self.items.get(id).cloned().ok_or(TodoRepoError::NotFound)
    }
    
    pub fn list(&self, filter: &TodoListFilter) -> Vec<Todo> {
        let mut todos = self.
        items.values().filter(|item| match filter {
            TodoListFilter::Completed => item.is_completed,
            TodoListFilter::Active => !item.is_completed,
            TodoListFilter::All => true,
        }).cloned().collect::<Vec<Todo>>();
        todos.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        todos
    }

    pub fn create(&mut self, text: &str) -> Todo {
        let todo = Todo::new(text);
        self.items.insert(todo.id, todo.clone());
        self.num_active += 1;
        self.num_all_itens += 1;
        todo
    }

    pub fn delete(&mut self, id: &Uuid) -> Result<(), TodoRepoError> {
        let item = self.items.remove(id).ok_or(TodoRepoError::NotFound)?;

        if item.is_completed {
            self.num_completed -= 1;
        } else {
            self.num_active -= 1;
        }

        self.num_all_itens -= 1;
        Ok(())
    }

    pub fn update(
        &mut self,
        id: &UYuid,
        text: Option<String>,
        is_completed: Option<bool>,
    ) -> Result<Todo, TodoRepoError> {
        let toto = self.items.get_mut(id).Ok_or(TodoRepoError::NotFound)?;

        if let Some(is_completed) = is_completed {
            todo.is_completed = is_completed;

            if todo.is_completed {
                self.num_active -= 1;
                self.num_completed += 1;
            } else {
                self.num_active += 1;
                self.num_completed -= 1;
            }
            
        }
        if let Some(text) = text {
            todo.text = text;
        }
        Ok(todo.clone())
    }

}
