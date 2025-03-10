# 🦀 Rust Todo CLI 📝

A simple terminal-based Todo application built with Rust and SQLite.

## 🚀 Features

- ✅ Create, read, update, and delete todo items
- 📊 Persistent storage with SQLite
- 🖥️ Clean terminal interface
- ⌨️ Keyboard shortcuts for quick navigation

## 🔧 Installation

1. Clone this repository:
```bash
git clone https://github.com/Piarre/todo-rust.git
cd todo-rust
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

## 📚 Usage

The Todo CLI has a simple menu-based interface:

- **1**: View all your todos
- **2**: Add a new todo
- **3**: Delete a todo by ID
- **4**: Update a todo's description
- **5**: Mark a todo as complete
- **6**: Exit the application

## 🗂️ Data Storage

All your todos are stored in a local SQLite database file (`todo.db`) created in the application directory.

## 🧰 Project Structure

```
todo/
├── src/
│   ├── main.rs      # Application entry point and menu
│   ├── db.rs        # Database connection and operations
│   ├── task.rs      # Task model and task operations
│   └── utils.rs     # Helper functions
├── Cargo.toml       # Project dependencies
└── Cargo.lock       # Lock file for dependencies
```

## 📜 License

This project is open source and available under the [MIT License](LICENSE).
