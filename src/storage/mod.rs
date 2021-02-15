use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_projects_list() -> Option<Vec<String>> {
    let path: &'static str = "/home/sc/dev/project-cli/PROJECTS.txt";

    let file = File::open(path).unwrap();
    let contents_reader = BufReader::new(file);
    let projects: Vec<String> = contents_reader.lines().map(|l| l.unwrap()).collect();

    Some(projects)
}

// fn parse_line(line: &str) -> Option<&str> {}
