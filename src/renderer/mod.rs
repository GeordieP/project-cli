use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum UIMode {
    Normal,
    Insert,
}

pub struct ScreenState {
    mode: UIMode,
    search_term: String,
    selected_project_index: usize,
    projects: Vec<String>,
}

impl ScreenState {
    pub fn new() -> Self {
        ScreenState {
            mode: UIMode::Normal,
            search_term: "".to_string(),
            selected_project_index: 0,
            projects: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
            ],
        }
    }
}

// state / types
// -------------------------
// pub

pub fn start() {
    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let num_lines = 10;

    // pad screen area to make sure we don't run off the end of the terminal
    // if we run off the end, screen clearing doesn't work right and each new render is printed starting on a new line
    render_safe_area(&mut stdout, num_lines);
    // store initial cursor position so we can re-draw each screen state from that position
    // Y position is offset by the # of safe area lines
    let initial_cursor_pos = termion::cursor::DetectCursorPos::cursor_pos(&mut stdout).unwrap();
    let initial_cursor_pos = (initial_cursor_pos.0, initial_cursor_pos.1 - num_lines);
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    // render initial state
    let mut state = ScreenState::new();
    render_state(&mut stdout, &state, initial_cursor_pos);

    for c in stdin.keys() {
        match &state.mode {
            UIMode::Normal => match c.unwrap() {
                Key::Char('k') | Key::Up => move_up_list(&mut state),
                Key::Char('j') | Key::Down => move_down_list(&mut state),
                Key::Char('i') | Key::Char('s') => {
                    let term = state.search_term.clone();
                    search_for(&mut state, term)
                }
                Key::Char('q') => {
                    write!(stdout, "{}", termion::cursor::Show).unwrap();
                    break;
                }
                _ => {}
            },
            UIMode::Insert => match c.unwrap() {
                Key::Esc => normal_mode(&mut state),
                Key::Backspace => {
                    let mut new_term = state.search_term.clone();
                    if new_term.len() > 0 {
                        new_term.truncate(&state.search_term.len() - 1);
                    }

                    search_for(&mut state, new_term);
                }
                Key::Char(chr) => {
                    let new_term = format!("{}{}", &state.search_term, chr);
                    search_for(&mut state, new_term);
                }
                _ => {}
            },
        }

        render_state(&mut stdout, &state, initial_cursor_pos);
        stdout.flush().unwrap();
    }

    stdout.flush().unwrap();
}

// pub
// -------------------------
// actions

fn move_up_list(state: &mut ScreenState) {
    let new_index = if state.projects.len() == 0 {
        0
    } else if state.selected_project_index == 0 {
        0
    } else {
        state.selected_project_index - 1
    };

    state.selected_project_index = new_index;
}

fn move_down_list(state: &mut ScreenState) {
    let new_index = state.selected_project_index + 1;
    let new_index = if state.projects.len() == 0 {
        0
    } else if new_index >= state.projects.len() {
        state.projects.len() - 1
    } else {
        new_index
    };

    state.selected_project_index = new_index;
}

fn search_for(state: &mut ScreenState, search_term: String) {
    state.mode = UIMode::Insert;
    state.search_term = search_term;
}

fn normal_mode(state: &mut ScreenState) {
    state.mode = UIMode::Normal;
}

// actions
// -------------------------
// rendering

fn render_state<W: Write>(stdout: &mut W, state: &ScreenState, initial_cursor_pos: (u16, u16)) {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(initial_cursor_pos.0, initial_cursor_pos.1),
        termion::clear::AfterCursor
    )
    .unwrap();

    render_header(stdout);
    render_search(stdout, &state);
    render_list(stdout, &state);
}

fn render_header(stdout: &mut impl Write) {
    write!(stdout, "  ---------- projects ----------\n\r",).unwrap();
}

fn render_search(stdout: &mut impl Write, state: &ScreenState) {
    write!(
        stdout,
        "{}\r\n",
        components::search_line(&state.mode, &state.search_term)
    )
    .unwrap();
}

fn render_list(stdout: &mut impl Write, state: &ScreenState) {
    state
        .projects
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let selected = i == state.selected_project_index;
            components::selectable_list_item(selected, label)
        })
        .for_each(|list_item| {
            write!(stdout, "{}{}", list_item, "\n\r").unwrap();
        });
}

fn render_safe_area<W: Write>(stdout: &mut W, num_buffer_lines: u16) {
    for _n in 0..num_buffer_lines {
        write!(stdout, "\n\r").unwrap();
    }
}

// rendering
// -------------------------
// components

mod components {
    use super::UIMode;

    pub fn search_line(mode: &UIMode, search_term: &String) -> String {
        let mode_indicator = if let UIMode::Insert = mode { "i" } else { "n" };

        let search_term = if search_term.len() == 0 {
            match mode {
                UIMode::Insert => {
                    format!(
                        "{}type to search{}",
                        termion::color::Fg(termion::color::LightBlack),
                        termion::color::Fg(termion::color::Reset)
                    )
                }

                UIMode::Normal => {
                    format!(
                        "{}press i to search{}",
                        termion::color::Fg(termion::color::LightBlack),
                        termion::color::Fg(termion::color::Reset)
                    )
                }
            }
        } else {
            search_term.clone()
        };

        format!("  [{}] {}", mode_indicator, search_term)
    }

    pub fn selectable_list_item(selected: bool, list_item_label: &str) -> String {
        if selected == true {
            format!(
                "→ {}{}{}",
                termion::color::Fg(termion::color::Yellow),
                list_item_label,
                termion::color::Fg(termion::color::Reset),
            )
        } else {
            format!(
                "  {}{}{}",
                termion::color::Fg(termion::color::Reset),
                list_item_label,
                termion::color::Fg(termion::color::Reset),
            )
        }
    }
}
