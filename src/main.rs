extern crate termion;

mod renderer;
mod termion_examples;

fn main() {
    // termionexamples::password_input();
    // termionexamples::async_input();
    // termionexamples::keys();
    // termion_examples::basic();
    // termion_examples::rainbow();

    renderer::start();
}
