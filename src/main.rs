extern crate lazy_static;
mod line;
mod progress;
mod request;

use line::Line;
use progress::Progress;
use request::Request;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let filename = "log";
    println!("reading file: '{}'...", filename);
    let log = fs::read_to_string(filename).expect("no file named \"log\" available");
    let _lines = log.lines().collect::<Vec<&str>>();
    let lines = _lines.iter().map(|l| Line::new(l)).collect::<Vec<Line>>();

    let relevant_ids = get_relevant_ids(&lines);
    println!("{} relevant IDs", relevant_ids.len());

    let requests = group_by_id(&lines, relevant_ids);
    println!("found {} errors", requests.len());

    let errors = count_unique_errors(&requests);
    println!("found {} unique errors", errors.len());

    println!("writing to file...");
    fs::write("requests.json", serde_json::to_string(&requests).unwrap())
        .expect("could not serialize requests!");

    fs::write(
        "unique_errors.json",
        serde_json::to_string(&errors).unwrap(),
    )
    .expect("could not unique serialize errors!");
    println!("done!");
}

fn get_relevant_ids(lines: &Vec<Line>) -> HashSet<&str> {
    let mut progress = Progress::new(lines.len());
    let mut relevant_ids = HashSet::new();
    lines.iter().for_each(|line| {
        progress.print_and_increment();
        if !line.is_relevant() {
            return;
        }
        if let Some(id) = &line.get_id() {
            relevant_ids.insert(*id);
        }
    });
    return relevant_ids;
}

fn group_by_id(lines: &Vec<Line>, relevant_ids: HashSet<&str>) -> Vec<Request> {
    let requests: RefCell<HashMap<&str, Request>> = RefCell::new(HashMap::new());
    println!("creating request objects...");
    relevant_ids
        .into_iter()
        .for_each(|id| drop(requests.borrow_mut().insert(id, Request::new())));

    let mut progress = Progress::new(lines.len());

    println!("grouping by id...");
    for line in lines {
        progress.print_and_increment();

        let Some(id) = line.get_id() else { continue; };
        if let Some(request) = requests.borrow_mut().get_mut(&id) {
            request.add_line(line.clone());
        }
    }

    return requests.into_inner().into_values().collect();
}

fn count_unique_errors(requests: &Vec<Request>) -> HashMap<&str, usize> {
    let mut errors = HashMap::new();
    let mut progress = Progress::new(requests.len());

    for request in requests {
        progress.print_and_increment();

        let Some(error) = request.get_error_message() else { continue; };
        let count = errors.entry(error).or_insert(0);
        *count += 1;
    }
    return errors;
}
