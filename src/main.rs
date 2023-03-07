mod line;
mod progress;
mod request;

use line::Line;
use progress::Progress;
use request::Request;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let (lines, relevant_ids) = get_lines_and_relevant_ids("log");
    println!("{} relevant IDs", relevant_ids.len());

    let requests = group_by_id(lines, relevant_ids);
    println!("found {} errors", requests.len());

    fs::write("requests.json", serde_json::to_string(&requests).unwrap())
        .expect("could not serialize requests!");
}

fn get_lines_and_relevant_ids(filename: &str) -> (Vec<Line>, HashSet<String>) {
    println!("reading file: '{}'...", filename);
    let log = fs::read_to_string(filename).expect("no file named \"log\" available");
    let lines = log.lines().collect::<Vec<&str>>();

    let mut progress = Progress::new(lines.len());
    let mut relevant_ids = HashSet::new();
    let lines = lines
        .iter()
        .map(|l| {
            progress.print_and_increment();
            let line = Line::new(l);
            if !line.is_relevant() {
                return line;
            }
            let Some(id) = line.get_id() else { return line; };
            relevant_ids.insert(id);
            return line;
        })
        .collect();

    return (lines, relevant_ids);
}

fn group_by_id(lines: Vec<Line>, relevant_ids: HashSet<String>) -> Vec<Request> {
    let mut requests: HashMap<String, Request> = HashMap::new();
    println!("creating request objects...");
    relevant_ids
        .into_iter()
        .for_each(|id| drop(requests.insert(id, Request::new())));

    let mut progress = Progress::new(lines.len());

    println!("grouping by id...");
    lines.into_iter().for_each(|line| {
        progress.print_and_increment();

        let Some(id) = line.get_id() else { return; };
        let Some(request) = requests.get_mut(&id) else { return; };
        request.add_line(line);
    });

    return requests.into_values().collect();
}
