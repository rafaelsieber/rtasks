# RTasks - Terminal Task Manager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)

A simple, efficient terminal-based task management application written in Rust with a vim-like interface.

![RTasks Demo](https://via.placeholder.com/600x400/1e1e1e/ffffff?text=RTasks+Screenshot)

## Features

- ✅ Create, Read, Update, Delete (CRUD) tasks
- 🔍 Navigate through tasks using arrow keys
- ✔️ Mark tasks as completed/incomplete
- 💾 Persistent storage (tasks saved to `~/.local/share/rtasks/tasks.json`)
- 🎨 Clean, intuitive terminal UI
- ⌨️ Keyboard-driven interface
- 🚀 Command-line interface for quick task addition
- 📋 List tasks from command line

## Controls

- **↑/↓ Arrow Keys**: Navigate through tasks
- **Space**: Toggle task completion status
- **A**: Add new task
- **E**: Edit selected task title
- **D**: Edit selected task description
- **Delete**: Remove selected task
- **Q**: Quit application
- **Esc**: Cancel current operation
- **Ctrl+C**: Force quit

## Installation

### From Source

1. Make sure you have Rust installed on your system ([Install Rust](https://rustup.rs/))
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/rtasks.git
   cd rtasks
   ```
3. Build and install:
   ```bash
   cargo build --release
   
   # Optional: Install globally
   cargo install --path .
   ```
4. Run:
   ```bash
   # If installed globally
   rtasks
   
   # Or run from source
   ./target/release/rtasks
   ```

### Pre-built Binaries

Check the [Releases](https://github.com/yourusername/rtasks/releases) page for pre-built binaries for your platform.

## Usage

### Interactive Mode
1. Run the application with `cargo run` or `./target/release/rtasks`
2. Use 'A' to add your first task
3. Navigate with arrow keys
4. Use Space to mark tasks as done
5. Use 'E' to edit task titles
6. Use 'D' to edit task descriptions
7. Use Delete to remove tasks
8. Press 'Q' to quit

### Command Line Mode
You can also add tasks directly from the command line without entering the interactive mode:

```bash
# Add a simple task
./target/release/rtasks -a "Fazer almoço"

# Add a task with description
./target/release/rtasks -a "Comprar mantimentos" -d "Leite, pão, ovos e frutas"

# List all tasks
./target/release/rtasks -l

# Show help
./target/release/rtasks --help
```

#### Command Line Options

- `-a, --add <TASK>`: Add a new task and exit
- `-d, --description <DESCRIPTION>`: Description for the task (used with -a)
- `-l, --list`: List all tasks and exit
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Data Storage

Tasks are automatically saved to `~/.local/share/rtasks/tasks.json`. The directory is created automatically when you add your first task. If the directory cannot be created, it falls back to `tasks.json` in the current directory.

## Dependencies

- `crossterm`: Cross-platform terminal manipulation
- `serde`: Serialization framework
- `serde_json`: JSON serialization
- `clap`: Command line argument parser

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Terminal interface powered by [crossterm](https://github.com/crossterm-rs/crossterm)
- Command-line parsing by [clap](https://github.com/clap-rs/clap)
