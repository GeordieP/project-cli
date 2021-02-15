extern crate termion;

mod renderer;
mod storage;

fn main() {
    let projects = storage::get_projects_list().expect("No projects");
    let mut state = renderer::ScreenState::new(projects);
    renderer::start(&mut state);
}
