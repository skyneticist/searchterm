use crossterm::{
    event::{self, KeyCode},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use jwalk::WalkDir; // Using jwalk for parallel file iteration
use std::io::{self};
use std::path::Path;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal
    terminal::enable_raw_mode()?;
    io::stdout().execute(Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut selected_index = 0;
    let current_path = Path::new(".");
    let all_files_and_dirs: Vec<String> = get_files_and_dirs(&current_path); // Get all files and directories

    let mut search_input = String::new(); // Variable to store search input

    loop {
        // // Filter: Create a filtered list based on search_input
        // let files_and_dirs: Vec<ListItem<'_>> = all_files_and_dirs
        //     .iter()
        //     .filter(|entry| search_input.is_empty() || entry.contains(&search_input))
        //     .map(|entry| ListItem::new(entry.clone()))
        //     .collect();
        // Filter: Create a filtered list based on search_input
        let filtered_files_and_dirs: Vec<String> = all_files_and_dirs
            .iter()
            .filter(|entry| search_input.is_empty() || entry.contains(&search_input))
            .cloned()
            .collect();

        let files_and_dirs: Vec<ListItem<'_>> = filtered_files_and_dirs
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let style = if index == selected_index {
                    Style::default().fg(Color::Yellow) // Style for selected item
                } else {
                    Style::default().fg(Color::LightGreen) // Style for other items
                };
                let styled_name = Span::styled(entry.clone(), style);
                ListItem::new(Spans::from(styled_name))
            })
            .collect();

        terminal.draw(|f| {
            // Define layout constraints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3), // Search/Filter bar
                        Constraint::Min(1),    // Paths/Files list
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Define color style for search/filter bar
            let search_bar_style = Style::default().fg(Color::Yellow).bg(Color::Black);

            // Create search/filter bar widget with color
            let search_bar = Block::default()
                .title("Search/Filter")
                .borders(Borders::ALL)
                .style(search_bar_style);

            // Create a paragraph widget to display the search input text
            let search_input_widget = Paragraph::new(search_input.as_ref()).block(search_bar);

            // Draw search/filter bar with search input text
            f.render_widget(search_input_widget, chunks[0]);

            // Define color style for paths/files list
            let files_list_style = Style::default().fg(Color::White).bg(Color::Blue);
            let files_list_widget = List::new(&*files_and_dirs)
                // .select(Some(selected_index))
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Files",
                            Style::default().fg(Color::Magenta), // Custom title color
                        ))
                        .borders(Borders::ALL)
                        .style(files_list_style),
                )
                .style(Style::default().fg(Color::LightGreen))
                .highlight_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray)) // Highlight selected item
                .highlight_symbol("> ");

            // Draw paths/files list
            f.render_widget(files_list_widget, chunks[1]);
        })?;

        // Listen for key events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        search_input.push(c); // Append typed character to search input
                    }
                    KeyCode::Backspace => {
                        search_input.pop(); // Remove last character from search input
                    }
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1; // Move selection up
                            println!("{}", selected_index);
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < files_and_dirs.len() - 1 {
                            selected_index += 1; // Move selection down
                        }
                    }
                    KeyCode::Esc => break, // Exit loop on Esc key
                    // Handle other keys and actions here
                    _ => {}
                }
            }
        }
    }

    // Restore the terminal state
    terminal::disable_raw_mode()?;
    Ok(())
}

// fn get_files_and_dirs(path: &Path) -> Vec<ListItem<'static>> {
//     WalkDir::new(path)
//         .into_iter()
//         .filter_map(Result::ok)
//         .map(|entry| {
//             let file_name = entry.file_name().to_string_lossy().into_owned();
//             ListItem::new(file_name) // Create a ListItem for each file name
//         })
//         .collect()
// }
fn get_files_and_dirs(path: &Path) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}
