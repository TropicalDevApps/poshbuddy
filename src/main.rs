use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Theme {
    name: String,
    download_url: String,
}

struct App {
    themes: Vec<String>,
    selected_index: usize,
    filter: String,
    themes_dir: PathBuf,
    profile_path: PathBuf,
    version: String,
}

impl App {
    fn new() -> Self {
        let home = dirs::home_dir().expect("Could not find home directory");
        let themes_dir = home.join(".poshthemes");
        let profile_path = if cfg!(windows) {
            home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1")
        } else {
            home.join(".config/powershell/Microsoft.PowerShell_profile.ps1")
        };

        App {
            themes: Vec::new(),
            selected_index: 0,
            filter: String::new(),
            themes_dir,
            profile_path,
            version: "0.2.0-rust".to_string(),
        }
    }

    fn filtered_themes(&self) -> Vec<&String> {
        self.themes
            .iter()
            .filter(|t| t.to_lowercase().contains(&self.filter.to_lowercase()))
            .collect()
    }

    fn apply_theme(&self, theme_name: &str) -> io::Result<()> {
        let theme_path = self.themes_dir.join(theme_name);
        let config_line = format!(
            "oh-my-posh init pwsh --config '{}' | Invoke-Expression",
            theme_path.display()
        );

        let content = if self.profile_path.exists() {
            fs::read_to_string(&self.profile_path)?
        } else {
            String::new()
        };

        let mut new_content = Vec::new();
        let mut found = false;
        let re = regex::Regex::new(r"(?i)^oh-my-posh init .*").unwrap();

        for line in content.lines() {
            if re.is_match(line) {
                new_content.push(config_line.clone());
                found = true;
            } else {
                new_content.push(line.to_string());
            }
        }

        if !found {
            if !content.is_empty() {
                new_content.push("".to_string());
            }
            new_content.push(config_line);
        }

        if let Some(parent) = self.profile_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.profile_path, new_content.join("\n"))?;
        Ok(())
    }
}

async fn fetch_themes() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/JanDeDobbeleer/oh-my-posh/contents/themes";
    let client = reqwest::Client::builder()
        .user_agent("PoshBuddy-Rust")
        .build()?;

    let resp = client
        .get(url)
        .send()
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;
    let themes = resp
        .into_iter()
        .filter_map(|v| v["name"].as_str().map(|s| s.to_string()))
        .filter(|s| s.ends_with(".omp.json"))
        .collect();

    Ok(themes)
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let filtered = app.filtered_themes();
    let items: Vec<ListItem> = filtered
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(t.as_str()).style(style)
        })
        .collect();

    let themes_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Themes v{}", app.version)),
        )
        .highlight_symbol("> ");

    f.render_widget(themes_list, chunks[0]);

    let preview_text = if !filtered.is_empty() {
        format!(
            "Previewing: {}\n\nFilter: {}\n\nPress [Enter] to apply theme\nPress [Esc] or [q] to quit",
            filtered[app.selected_index], app.filter
        )
    } else {
        format!("No themes found matching: {}\n\nPress [Esc] or [q] to quit", app.filter)
    };

    let preview = Paragraph::new(preview_text)
        .block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(preview, chunks[1]);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    // In a real app we might show a loading screen while this happens
    app.themes = fetch_themes().await.unwrap_or_default();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down => {
                        let total = app.filtered_themes().len();
                        if total > 0 {
                            app.selected_index = (app.selected_index + 1) % total;
                        }
                    }
                    KeyCode::Up => {
                        let total = app.filtered_themes().len();
                        if total > 0 {
                            app.selected_index = (app.selected_index + total - 1) % total;
                        }
                    }
                    KeyCode::Enter => {
                        let filtered = app.filtered_themes();
                        if !filtered.is_empty() {
                            let theme_name = filtered[app.selected_index];
                            app.apply_theme(theme_name)?;
                            break;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.filter.push(c);
                        app.selected_index = 0;
                    }
                    KeyCode::Backspace => {
                        app.filter.pop();
                        app.selected_index = 0;
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
