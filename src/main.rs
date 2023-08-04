use crossterm::{
    event::{self, KeyCode},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use jwalk::WalkDir;
use rayon::vec; // parallel iteration
use std::path::Path;
use std::{
    env,
    io::{self},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

fn draw_search_bar() {
    // Define color style for search/filter bar
    let search_bar_style = Style::default().fg(Color::Yellow);

    // Create search/filter bar widget with color
    let search_bar = Block::default()
        .title("Search/Filter")
        .borders(Borders::ALL)
        .style(search_bar_style);
}

fn draw_main_window(f: &mut Frame<'_, CrosstermBackend<io::Stdout>>) -> Vec<Rect> {
    // Define layout constraints
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Search/Filter bar
                Constraint::Min(1),
                Constraint::Max(2), // Paths/Files list
            ]
            .as_ref(),
        )
        .split(f.size())
}

fn dynamic_draw() {
    terminal.draw(|f: &mut Frame<'_, CrosstermBackend<io::Stdout>>| {
        // Define layout constraints
        // let chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(
        //         [
        //             Constraint::Length(3), // Search/Filter bar
        //             Constraint::Min(1),
        //             Constraint::Max(2), // Paths/Files list
        //         ]
        //         .as_ref(),
        //     )
        //     .split(f.size());

        // draw_main_window(f);

        // // Define color style for search/filter bar
        // let search_bar_style = Style::default().fg(Color::Yellow);

        // // Create search/filter bar widget with color
        // let search_bar = Block::default()
        //     .title("Search/Filter")
        //     .borders(Borders::ALL)
        //     .style(search_bar_style);

        // Create a paragraph widget to display the search input text
        let search_input_widget = Paragraph::new(search_input.as_ref()).block(search_bar);

        // Draw search/filter bar with search input text
        f.render_widget(search_input_widget, chunks[0]);

        // Define color style for paths/files list
        let files_list_style = Style::default().fg(Color::Blue);
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
            .highlight_symbol("> ");

        let sub_text_style = Style::default().fg(Color::White);

        let sub_menu_items: Vec<ListItem> = vec![
            ListItem::new("copy").style(sub_text_style),
            ListItem::new("reveal").style(sub_text_style),
        ];

        // Calculate sub-menu position based on selected_index
        let sub_menu_y = chunks[1].y + selected_index as u16; // Example offset
        let sub_menu_x = chunks[1].x + 21; // set to char length?

        let sub_menu_items_length = &sub_menu_items.len();
        // Create sub-menu widget (e.g., List or other suitable widget)
        let sub_menu_style = Style::default().fg(Color::Green).bg(Color::White);
        let sub_menu_widget = List::new(sub_menu_items)
            .block(Block::default().borders(Borders::ALL).style(sub_menu_style))
            .style(sub_menu_style)
            .start_corner(tui::layout::Corner::TopLeft)
            .highlight_style(search_bar_style)
            .highlight_symbol("> ");

        if show_sub_menu {
            // Render sub-menu at calculated position
            // f.render_widget(sub_menu_widget, chunks[2]);
            f.render_widget(
                sub_menu_widget,
                Rect::new(sub_menu_x, sub_menu_y, 20, *sub_menu_items_length as u16),
            );
        }

        // Draw paths/files list
        f.render_widget(files_list_widget, chunks[1]);
});

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the terminal
    terminal::enable_raw_mode()?;
    io::stdout().execute(Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut selected_index = 0;
    let mut sub_selected_index = 0;
    let current_path = env::current_dir().unwrap();
    // let current_path = Path::new(".");
    let all_files_and_dirs: Vec<String> = get_files_and_dirs(&current_path); // Get all files and directories

    let mut search_input = String::new(); // Variable to store search input
    let mut show_sub_menu = false; // Whether to show the sub-menu
                                   // let _sub_menu_items: Vec<&str> = vec!["copy", "reveal"]; // Content of the sub-menu

    let mut search_changed: bool = true; // intialize file list with true
    let mut filtered_files_and_dirs: Vec<String> = vec![];
    let mut files_and_dirs: Vec<ListItem<'_>>;
    let mut previous_search_input = String::new();

    let mut chunks: Vec<Rect> = vec![];
    let _ = terminal.draw(|f: &mut Frame<'_, CrosstermBackend<io::Stdout>>| {
        chunks = draw_main_window(f);
    });

    loop {
        if search_changed {
            if search_input.starts_with(&previous_search_input) {
                // Character added, filter the existing filtered list
                filtered_files_and_dirs.retain(|entry| entry.contains(&search_input));
            } else {
                // Character deleted, filter from the original list
                filtered_files_and_dirs = all_files_and_dirs
                    .iter()
                    .filter(|entry| entry.contains(&search_input))
                    .cloned()
                    .collect();
            }
            previous_search_input = search_input.clone();
            search_changed = false;
        }

        //  filtered_files_and_dirs = all_files_and_dirs
        //     .iter()
        //     .filter(|entry| search_input.is_empty() || entry.contains(&search_input))
        //     .cloned()
        //     .collect();

        // let files_and_dirs: Vec<ListItem<'_>>;

        files_and_dirs = filtered_files_and_dirs
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let style = if index == selected_index {
                    Style::default().fg(Color::LightRed).bg(Color::Blue) // selected file
                } else {
                    Style::default().fg(Color::LightGreen)
                };
                let styled_name = Span::styled(entry.clone(), style);
                ListItem::new(Spans::from(styled_name))
            })
            .collect();

        // terminal.draw(|f: &mut Frame<'_, CrosstermBackend<io::Stdout>>| {
        //     // Define layout constraints
        //     // let chunks = Layout::default()
        //     //     .direction(Direction::Vertical)
        //     //     .constraints(
        //     //         [
        //     //             Constraint::Length(3), // Search/Filter bar
        //     //             Constraint::Min(1),
        //     //             Constraint::Max(2), // Paths/Files list
        //     //         ]
        //     //         .as_ref(),
        //     //     )
        //     //     .split(f.size());

        //     // draw_main_window(f);

        //     // // Define color style for search/filter bar
        //     // let search_bar_style = Style::default().fg(Color::Yellow);

        //     // // Create search/filter bar widget with color
        //     // let search_bar = Block::default()
        //     //     .title("Search/Filter")
        //     //     .borders(Borders::ALL)
        //     //     .style(search_bar_style);

        //     // Create a paragraph widget to display the search input text
        //     let search_input_widget = Paragraph::new(search_input.as_ref()).block(search_bar);

        //     // Draw search/filter bar with search input text
        //     f.render_widget(search_input_widget, chunks[0]);

        //     // Define color style for paths/files list
        //     let files_list_style = Style::default().fg(Color::Blue);
        //     let files_list_widget = List::new(&*files_and_dirs)
        //         // .select(Some(selected_index))
        //         .block(
        //             Block::default()
        //                 .title(Span::styled(
        //                     "Files",
        //                     Style::default().fg(Color::Magenta), // Custom title color
        //                 ))
        //                 .borders(Borders::ALL)
        //                 .style(files_list_style),
        //         )
        //         .highlight_symbol("> ");

        //     let sub_text_style = Style::default().fg(Color::White);

        //     let sub_menu_items: Vec<ListItem> = vec![
        //         ListItem::new("copy").style(sub_text_style),
        //         ListItem::new("reveal").style(sub_text_style),
        //     ];

        //     // Calculate sub-menu position based on selected_index
        //     let sub_menu_y = chunks[1].y + selected_index as u16; // Example offset
        //     let sub_menu_x = chunks[1].x + 21; // set to char length?

        //     let sub_menu_items_length = &sub_menu_items.len();
        //     // Create sub-menu widget (e.g., List or other suitable widget)
        //     let sub_menu_style = Style::default().fg(Color::Green).bg(Color::White);
        //     let sub_menu_widget = List::new(sub_menu_items)
        //         .block(Block::default().borders(Borders::ALL).style(sub_menu_style))
        //         .style(sub_menu_style)
        //         .start_corner(tui::layout::Corner::TopLeft)
        //         .highlight_style(search_bar_style)
        //         .highlight_symbol("> ");

        //     if show_sub_menu {
        //         // Render sub-menu at calculated position
        //         // f.render_widget(sub_menu_widget, chunks[2]);
        //         f.render_widget(
        //             sub_menu_widget,
        //             Rect::new(sub_menu_x, sub_menu_y, 20, *sub_menu_items_length as u16),
        //         );
        //     }

        //     // Draw paths/files list
        //     f.render_widget(files_list_widget, chunks[1]);
        })?;
        // terminal.flush()?;
        // Listen for key events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        search_input.push(c); // Append typed character to search input
                        search_changed = true;
                    }
                    KeyCode::Backspace => {
                        search_input.pop(); // Remove last character from search input
                        search_changed = true;
                    }
                    KeyCode::Up => {
                        if !show_sub_menu {
                            if selected_index > 0 {
                                selected_index -= 1; // Move selection up
                            }
                        } else {
                            if sub_selected_index > 0 {
                                sub_selected_index -= 1;
                            }
                        }
                    }
                    KeyCode::Down => {
                        if !show_sub_menu {
                            if selected_index < files_and_dirs.len() - 1 {
                                selected_index += 1; // Move selection down
                            }
                        } else {
                            if sub_selected_index < 1 {
                                sub_selected_index += 1;
                            }
                        }
                    }
                    KeyCode::Right => {
                        show_sub_menu = !show_sub_menu;
                    }
                    KeyCode::Left => {
                        show_sub_menu = false; // Toggle sub-menu visibility
                    }
                    KeyCode::Enter => {}
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
        .follow_links(false)
        .skip_hidden(true)
        // .process_read_dir(process_by)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path().display().to_string())
        .collect()
}

/////////
// use crossterm::{
//     event::{self, KeyCode},
//     terminal::{self, Clear, ClearType},
//     ExecutableCommand,
// };
// use jwalk::WalkDir; // parallel iteration
// use std::path::Path;
// use std::{
//     collections::HashMap,
//     io::{self},
//     path::PathBuf,
// };
// use tui::{
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout, Rect},
//     style::{Color, Style},
//     text::{Span, Spans},
//     widgets::{Block, Borders, List, ListItem, Paragraph},
//     Terminal,
// };

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Initialize the terminal
//     terminal::enable_raw_mode()?;
//     io::stdout().execute(Clear(ClearType::All))?;

//     let backend = CrosstermBackend::new(io::stdout());
//     let mut terminal = Terminal::new(backend)?;
//     terminal.clear()?;

//     let mut selected_index = 0;
//     let mut sub_selected_index = 0;
//     let current_path = Path::new(".");
//     let all_files_and_dirs: HashMap<PathBuf, String> = get_files_and_dirs(&current_path); // Get all files and directories

//     let mut search_input = String::new(); // Variable to store search input
//     let mut show_sub_menu = false; // Whether to show the sub-menu
//                                    // let _sub_menu_items: Vec<&str> = vec!["copy", "reveal"]; // Content of the sub-menu

//     loop {
//         let filtered_files_and_dirs: HashMap<PathBuf, String> = all_files_and_dirs
//             .iter()
//             .filter(|(_, entry)| search_input.is_empty() || entry.contains(&search_input))
//             .map(|(path, filename)| (path.clone(), filename.clone()))
//             .collect();

//         let files_and_dirs: Vec<ListItem<'_>> = filtered_files_and_dirs
//             .iter()
//             .enumerate()
//             .map(|(index, (_, filename))| {
//                 let style = if index == selected_index {
//                     Style::default().fg(Color::LightRed).bg(Color::Blue) // selected file
//                 } else {
//                     Style::default().fg(Color::LightGreen)
//                 };
//                 let styled_name = Span::styled(filename, style);
//                 ListItem::new(Spans::from(styled_name))
//             })
//             .collect();

//         terminal.draw(|f| {
//             // Define layout constraints
//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .constraints(
//                     [
//                         Constraint::Length(3), // Search/Filter bar
//                         Constraint::Min(1),
//                         Constraint::Max(2), // Paths/Files list
//                     ]
//                     .as_ref(),
//                 )
//                 .split(f.size());

//             // Define color style for search/filter bar
//             let search_bar_style = Style::default().fg(Color::Yellow).bg(Color::Black);

//             // Create search/filter bar widget with color
//             let search_bar = Block::default()
//                 .title("Search/Filter")
//                 .borders(Borders::ALL)
//                 .style(search_bar_style);

//             // Create a paragraph widget to display the search input text
//             let search_input_widget = Paragraph::new(search_input.as_ref()).block(search_bar);

//             // Draw search/filter bar with search input text
//             f.render_widget(search_input_widget, chunks[0]);

//             // Define color style for paths/files list
//             let files_list_style = Style::default().fg(Color::Blue).bg(Color::DarkGray);
//             let files_list_widget = List::new(&*files_and_dirs)
//                 // .select(Some(selected_index))
//                 .block(
//                     Block::default()
//                         .title(Span::styled(
//                             "Files",
//                             Style::default().fg(Color::Magenta), // Custom title color
//                         ))
//                         .borders(Borders::ALL)
//                         .style(files_list_style),
//                 )
//                 .highlight_symbol("> ");

//             let sub_text_style = Style::default().fg(Color::White);

//             let sub_menu_items: Vec<ListItem> = vec![
//                 ListItem::new("copy").style(sub_text_style),
//                 ListItem::new("reveal").style(sub_text_style),
//             ];

//             // Calculate sub-menu position based on selected_index
//             let sub_menu_y = chunks[1].y + selected_index as u16; // Example offset
//             let sub_menu_x = chunks[1].x + 21; // set to char length?

//             let sub_menu_items_length = &sub_menu_items.len();
//             // Create sub-menu widget (e.g., List or other suitable widget)
//             let sub_menu_style = Style::default().fg(Color::Green).bg(Color::White);
//             let sub_menu_widget = List::new(sub_menu_items)
//                 .block(Block::default().borders(Borders::ALL).style(sub_menu_style))
//                 .style(sub_menu_style)
//                 .start_corner(tui::layout::Corner::TopLeft);
//             // .highlight_style(search_bar_style)
//             // .highlight_symbol("> ");

//             if show_sub_menu {
//                 // Render sub-menu at calculated position
//                 // f.render_widget(sub_menu_widget, chunks[2]);
//                 f.render_widget(
//                     sub_menu_widget,
//                     Rect::new(sub_menu_x, sub_menu_y, 20, *sub_menu_items_length as u16),
//                 );
//             }

//             // Draw paths/files list
//             f.render_widget(files_list_widget, chunks[1]);
//         })?;
//         // terminal.flush()?;
//         // Listen for key events
//         if event::poll(std::time::Duration::from_millis(100))? {
//             if let event::Event::Key(key_event) = event::read()? {
//                 match key_event.code {
//                     KeyCode::Char(c) => {
//                         search_input.push(c); // Append typed character to search input
//                     }
//                     KeyCode::Backspace => {
//                         search_input.pop(); // Remove last character from search input
//                     }
//                     KeyCode::Up => {
//                         if !show_sub_menu {
//                             if selected_index > 0 {
//                                 selected_index -= 1; // Move selection up
//                             }
//                         } else {
//                             if sub_selected_index > 0 {
//                                 sub_selected_index -= 1;
//                             }
//                         }
//                     }
//                     KeyCode::Down => {
//                         if !show_sub_menu {
//                             if selected_index < files_and_dirs.len() - 1 {
//                                 selected_index += 1; // Move selection down
//                             }
//                         } else {
//                             if sub_selected_index < 1 {
//                                 sub_selected_index += 1;
//                             }
//                         }
//                     }
//                     KeyCode::Right => {
//                         show_sub_menu = !show_sub_menu;
//                     }
//                     KeyCode::Left => {
//                         show_sub_menu = false; // Toggle sub-menu visibility
//                     }
//                     KeyCode::Enter => {}
//                     KeyCode::Esc => break, // Exit loop on Esc key
//                     // Handle other keys and actions here
//                     _ => {}
//                 }
//             }
//         }
//     }

//     // Restore the terminal state
//     terminal::disable_raw_mode()?;
//     Ok(())
// }

// // fn get_files_and_dirs(path: &Path) -> Vec<ListItem<'static>> {
// //     WalkDir::new(path)
// //         .into_iter()
// //         .filter_map(Result::ok)
// //         .map(|entry| {
// //             let file_name = entry.file_name().to_string_lossy().into_owned();
// //             ListItem::new(file_name) // Create a ListItem for each file name
// //         })
// //         .collect()
// // }
// fn get_files_and_dirs(path: &Path) -> HashMap<PathBuf, String> {
//     WalkDir::new(path)
//         .into_iter()
//         .filter_map(Result::ok)
//         .map(|entry| {
//             (
//                 entry.path().to_path_buf(),
//                 entry.file_name().to_string_lossy().into_owned(),
//             )
//         })
//         .collect()
// }
