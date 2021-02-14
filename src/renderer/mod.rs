use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum UIMode {
    Normal,
    Search(String),
}

pub struct ScreenState {
    mode: UIMode,
    selected_project_index: usize,
    projects: Vec<String>,
}

impl ScreenState {
    pub fn new() -> Self {
        ScreenState {
            mode: UIMode::Normal,
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
    write_safe_area(&mut stdout, num_lines);
    // store initial cursor position so we can re-draw each screen state from that position
    // Y position is offset by the # of safe area lines
    let initial_cursor_pos = termion::cursor::DetectCursorPos::cursor_pos(&mut stdout).unwrap();
    let initial_cursor_pos = (initial_cursor_pos.0, initial_cursor_pos.1 - num_lines);
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    // render initial state
    let mut state = ScreenState::new();
    render_state(&mut stdout, &state, initial_cursor_pos);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('k') | Key::Up => move_up_list(&mut state),
            Key::Char('j') | Key::Down => move_down_list(&mut state),
            Key::Char('q') => {
                write!(stdout, "{}", termion::cursor::Show).unwrap();
                break;
            }
            _ => {}
        }

        render_state(&mut stdout, &state, initial_cursor_pos);
        stdout.flush().unwrap();
    }

    stdout.flush().unwrap();
}

// pub
// -------------------------
// mutating state

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

// mutating state
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

    write_header(stdout);
    write_search_line(stdout, &state);
    write_list_items(stdout, &state.projects, state.selected_project_index);
}

fn write_line(stdout: &mut impl Write) {
    write!(stdout, "\n\r").unwrap();
}

fn write_header(stdout: &mut impl Write) {
    write!(stdout, "  ---------- projects ----------",).unwrap();
    write_line(stdout);
}

fn write_search_line(stdout: &mut impl Write, state: &ScreenState) {
    write!(stdout, "{}\r\n", components::search_line(&state.mode)).unwrap();
}

fn write_list_items<W: Write>(
    stdout: &mut W,
    projects_list: &Vec<String>,
    selected_item_index: usize,
) {
    projects_list
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let selected = i == selected_item_index;
            components::selectable_list_item(selected, label)
        })
        .for_each(|list_item| {
            write!(stdout, "{}{}", list_item, "\n\r").unwrap();
        });
}

fn write_safe_area<W: Write>(stdout: &mut W, num_buffer_lines: u16) {
    for _n in 0..num_buffer_lines {
        write!(stdout, "\n\r").unwrap();
    }
}

mod components {
    use super::UIMode;

    pub fn search_line(mode: &UIMode) -> String {
        match mode {
            UIMode::Normal => {
                format!(
                    "  [N] {}(press i to search){}",
                    termion::color::Fg(termion::color::LightBlack),
                    termion::color::Fg(termion::color::Reset)
                )
            }
            UIMode::Search(term) => {
                format!("  [S] {}", term,)
            }
        }
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
