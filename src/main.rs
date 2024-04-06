struct Todo {
    content: String,
    completed: bool,
}

impl Todo {
    fn new(content: String) -> Self {
        Self {
            content,
            completed: false,
        }
    }
}

struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Todos {
        Todos { todos: Vec::new() }
    }

    fn addTodo(mut self, content: String) {
        let todo = Todo::new(String::from("hello"));
        self.todos.push(todo)
    }
}

fn main() {}
