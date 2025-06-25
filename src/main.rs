use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, ClearType},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;
use clap::{Arg, Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Debug)]
struct App {
    tasks: Vec<Task>,
    selected_index: usize,
    mode: AppMode,
    input_buffer: String,
    next_id: usize,
}

#[derive(Debug, PartialEq)]
enum AppMode {
    Normal,
    AddTask,
    EditTask,
    AddDescription,
    EditDescription,
}

impl App {
    fn new() -> Self {
        let mut app = App {
            tasks: Vec::new(),
            selected_index: 0,
            mode: AppMode::Normal,
            input_buffer: String::new(),
            next_id: 1,
        };
        app.load_tasks();
        app
    }

    fn get_data_file_path() -> PathBuf {
        if let Some(home_dir) = std::env::var_os("HOME") {
            let data_dir = PathBuf::from(home_dir).join(".local").join("share").join("rtasks");
            // Create the directory if it doesn't exist
            if let Err(_) = fs::create_dir_all(&data_dir) {
                // Fallback to current directory if we can't create the data directory
                return PathBuf::from("tasks.json");
            }
            data_dir.join("tasks.json")
        } else {
            // Fallback to current directory
            PathBuf::from("tasks.json")
        }
    }

    fn load_tasks(&mut self) {
        let data_file = Self::get_data_file_path();
        
        // Check if there's an old tasks.json in current directory to migrate
        let old_file = PathBuf::from("tasks.json");
        if old_file.exists() && !data_file.exists() {
            if let Ok(data) = fs::read_to_string(&old_file) {
                // Try to write to new location
                if let Ok(_) = fs::write(&data_file, &data) {
                    // Successfully migrated, remove old file
                    let _ = fs::remove_file(&old_file);
                    eprintln!("Migrated tasks from ./tasks.json to {}", data_file.display());
                }
            }
        }
        
        if data_file.exists() {
            if let Ok(data) = fs::read_to_string(&data_file) {
                if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&data) {
                    self.tasks = tasks;
                    if let Some(max_id) = self.tasks.iter().map(|t| t.id).max() {
                        self.next_id = max_id + 1;
                    }
                }
            }
        }
    }

    fn save_tasks(&self) {
        let data_file = Self::get_data_file_path();
        if let Ok(data) = serde_json::to_string_pretty(&self.tasks) {
            let _ = fs::write(&data_file, data);
        }
    }

    fn add_task(&mut self, title: String, description: String) {
        let task = Task {
            id: self.next_id,
            title,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        self.save_tasks();
    }

    fn delete_task(&mut self) {
        if !self.tasks.is_empty() && self.selected_index < self.tasks.len() {
            self.tasks.remove(self.selected_index);
            if self.selected_index >= self.tasks.len() && !self.tasks.is_empty() {
                self.selected_index = self.tasks.len() - 1;
            }
            self.save_tasks();
        }
    }

    fn toggle_task(&mut self) {
        if !self.tasks.is_empty() && self.selected_index < self.tasks.len() {
            self.tasks[self.selected_index].completed = !self.tasks[self.selected_index].completed;
            self.save_tasks();
        }
    }

    fn edit_current_task(&mut self, title: String) {
        if !self.tasks.is_empty() && self.selected_index < self.tasks.len() {
            self.tasks[self.selected_index].title = title;
            self.save_tasks();
        }
    }

    fn edit_current_description(&mut self, description: String) {
        if !self.tasks.is_empty() && self.selected_index < self.tasks.len() {
            self.tasks[self.selected_index].description = description;
            self.save_tasks();
        }
    }

    fn move_up(&mut self) {
        if !self.tasks.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        if !self.tasks.is_empty() && self.selected_index < self.tasks.len() - 1 {
            self.selected_index += 1;
        }
    }
}

fn draw_ui(app: &App) -> std::io::Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    
    // Get terminal size
    let (cols, rows) = terminal::size()?;
    
    // Draw top header bar (vim-like)
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    execute!(stdout(), SetBackgroundColor(Color::Blue))?;
    execute!(stdout(), SetForegroundColor(Color::White))?;
    
    let header_text = " RTasks - Terminal Task Manager";
    let padding = " ".repeat((cols as usize).saturating_sub(header_text.len()));
    execute!(stdout(), Print(format!("{}{}", header_text, padding)))?;
    
    execute!(stdout(), SetBackgroundColor(Color::Reset))?;
    execute!(stdout(), ResetColor)?;
    
    // Calculate content area (from line 1 to rows-2, leaving space for header and footer)
    let content_start = 2u16;
    let content_end = rows.saturating_sub(2);
    let mut current_line = content_start;

    // Instructions/input area based on mode
    if app.mode != AppMode::Normal {
        let instructions = match app.mode {
            AppMode::AddTask => "Adding new task. Type title and press Enter (Esc to cancel):",
            AppMode::EditTask => "Editing task title. Type new title and press Enter (Esc to cancel):",
            AppMode::AddDescription => "Adding description. Type description and press Enter (Esc to cancel):",
            AppMode::EditDescription => "Editing description. Type new description and press Enter (Esc to cancel):",
            _ => "",
        };
        
        execute!(
            stdout(),
            cursor::MoveTo(0, current_line),
            SetForegroundColor(Color::Yellow),
            Print(instructions),
            ResetColor
        )?;
        current_line += 1;

        // Input field
        execute!(
            stdout(),
            cursor::MoveTo(0, current_line),
            SetForegroundColor(Color::Green),
            Print(format!("> {}", app.input_buffer)),
            ResetColor
        )?;
        current_line += 2;
    }

    // Task list
    if app.tasks.is_empty() {
        execute!(
            stdout(),
            cursor::MoveTo(0, current_line),
            SetForegroundColor(Color::DarkGrey),
            Print("No tasks yet. Press 'A' to add your first task!"),
            ResetColor
        )?;
    } else {
        for (index, task) in app.tasks.iter().enumerate() {
            // Stop drawing if we've reached the bottom of content area
            if current_line >= content_end {
                break;
            }
            
            let is_selected = index == app.selected_index && app.mode == AppMode::Normal;
            let status_symbol = if task.completed { "[X]" } else { "[ ]" };
            
            execute!(stdout(), cursor::MoveTo(0, current_line))?;
            
            if is_selected {
                execute!(stdout(), SetForegroundColor(Color::Black))?;
                execute!(stdout(), SetBackgroundColor(Color::White))?;
            } else if task.completed {
                execute!(stdout(), SetForegroundColor(Color::DarkGrey))?;
            } else {
                execute!(stdout(), SetForegroundColor(Color::White))?;
            }

            let task_text = format!("{} {} {}", status_symbol, task.id, task.title);
            execute!(stdout(), Print(task_text))?;
            
            if !task.description.is_empty() {
                execute!(stdout(), SetForegroundColor(Color::DarkGrey))?;
                execute!(stdout(), Print(format!(" - {}", task.description)))?;
            }
            
            execute!(stdout(), SetBackgroundColor(Color::Reset))?;
            execute!(stdout(), ResetColor)?;
            current_line += 1;
        }
    }

    // Draw bottom status bar (vim-like)
    execute!(stdout(), cursor::MoveTo(0, rows - 1))?;
    execute!(stdout(), SetBackgroundColor(Color::DarkBlue))?;
    execute!(stdout(), SetForegroundColor(Color::White))?;
    
    let status_text = if app.mode == AppMode::Normal {
        " Controls: ↑↓ Navigate | Space: Toggle | A: Add | E: Edit | D: Edit Desc | Del: Delete | Q: Quit"
    } else {
        " Press Enter to confirm | Esc to cancel"
    };
    
    let status_padding = " ".repeat((cols as usize).saturating_sub(status_text.len()));
    execute!(stdout(), Print(format!("{}{}", status_text, status_padding)))?;
    
    execute!(stdout(), SetBackgroundColor(Color::Reset))?;
    execute!(stdout(), ResetColor)?;

    stdout().flush()?;
    Ok(())
}

fn handle_input(app: &mut App) -> std::io::Result<bool> {
    if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
        match app.mode {
            AppMode::Normal => match code {
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(false),
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Char(' ') => app.toggle_task(),
                KeyCode::Char('a') | KeyCode::Char('A') => {
                    app.mode = AppMode::AddTask;
                    app.input_buffer.clear();
                }
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    if !app.tasks.is_empty() {
                        app.mode = AppMode::EditTask;
                        app.input_buffer = app.tasks[app.selected_index].title.clone();
                    }
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    if !app.tasks.is_empty() {
                        app.mode = AppMode::EditDescription;
                        app.input_buffer = app.tasks[app.selected_index].description.clone();
                    }
                }
                KeyCode::Delete => app.delete_task(),
                _ => {}
            },
            AppMode::AddTask | AppMode::EditTask | AppMode::AddDescription | AppMode::EditDescription => {
                match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                        app.input_buffer.clear();
                    }
                    KeyCode::Enter => {
                        let input = app.input_buffer.trim().to_string();
                        if !input.is_empty() {
                            match app.mode {
                                AppMode::AddTask => {
                                    app.mode = AppMode::AddDescription;
                                    // Store the title temporarily
                                    let title = app.input_buffer.clone();
                                    app.input_buffer.clear();
                                    // We'll need to store this title somewhere temporary
                                    // For now, let's add the task with empty description
                                    app.add_task(title, String::new());
                                    app.mode = AppMode::Normal;
                                }
                                AppMode::EditTask => {
                                    app.edit_current_task(input);
                                    app.mode = AppMode::Normal;
                                }
                                AppMode::AddDescription => {
                                    // This case won't happen with current flow
                                    app.mode = AppMode::Normal;
                                }
                                AppMode::EditDescription => {
                                    app.edit_current_description(input);
                                    app.mode = AppMode::Normal;
                                }
                                _ => {}
                            }
                        } else {
                            app.mode = AppMode::Normal;
                        }
                        app.input_buffer.clear();
                    }
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    KeyCode::Char(c) => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            match c {
                                'c' => return Ok(false),
                                _ => {}
                            }
                        } else {
                            app.input_buffer.push(c);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(true)
}

fn main() -> std::io::Result<()> {
    let matches = Command::new("rtasks")
        .version("0.1.0")
        .about("Terminal Task Manager")
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .value_name("TASK")
                .help("Add a new task and exit")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .value_name("DESCRIPTION")
                .help("Description for the task (used with -a)")
                .action(clap::ArgAction::Set)
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all tasks and exit")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Create app instance
    let mut app = App::new();

    // Handle command-line arguments
    if let Some(task_title) = matches.get_one::<String>("add") {
        let description = matches.get_one::<String>("description")
            .map(|s| s.to_string())
            .unwrap_or_default();
        
        app.add_task(task_title.clone(), description);
        println!("✅ Task added: {}", task_title);
        return Ok(());
    }

    if matches.get_flag("list") {
        if app.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            println!("📋 Your tasks:");
            for task in &app.tasks {
                let status = if task.completed { "✅" } else { "⬜" };
                let desc = if task.description.is_empty() {
                    String::new()
                } else {
                    format!(" - {}", task.description)
                };
                println!("{} [{}] {}{}", status, task.id, task.title, desc);
            }
        }
        return Ok(());
    }

    // Show data location on first run
    let data_file = App::get_data_file_path();
    if !data_file.exists() {
        eprintln!("RTasks data will be stored at: {}", data_file.display());
        eprintln!("Press any key to continue...");
        let _ = event::read();
    }
    
    // Enable raw mode for terminal
    terminal::enable_raw_mode()?;
    
    // Main application loop
    loop {
        draw_ui(&app)?;
        
        if !handle_input(&mut app)? {
            break;
        }
    }
    
    // Restore terminal
    execute!(stdout(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    terminal::disable_raw_mode()?;
    println!("Thanks for using RTasks! 👋");
    
    Ok(())
}
