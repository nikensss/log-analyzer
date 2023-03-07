mod progress;

use progress::Progress;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("reading file: 'log'...");
    let log = fs::read_to_string("log").expect("No file named \"log\" available");
    let lines: Vec<&str> = log.lines().collect();
    println!("{} logs", lines.len());

    let relevant_ids = find_relevant_request_ids(&lines);
    let grouped_by_id = group_by_request_id(&lines, &relevant_ids);
    let (unique_errors, unknown_errors) = get_unique_error_messages(&grouped_by_id);

    println!("{} errors", &grouped_by_id.len());
    println!("{} unknown errors", &unknown_errors.len());
    println!("{} unique errors", &unique_errors.len());

    fs::write(
        "unknown_errors.json",
        serde_json::to_string(&unknown_errors).unwrap(),
    )
    .expect("Could not serialize unknown errors!");

    fs::write(
        "unique_errors.json",
        serde_json::to_string(&unique_errors).unwrap(),
    )
    .expect("Could not serialize error messages!");

    fs::write("by_id.json", serde_json::to_string(&grouped_by_id).unwrap())
        .expect("Could not serialize grouped logs!");
}

fn find_relevant_request_ids<'a>(lines: &'a Vec<&'a str>) -> HashSet<&'a str> {
    println!("finding relevant request IDs...");

    let ignorable_errors = vec![
        "\"errorStatus\":401,",
        "\"errorStatus\":404,",
        "\"statusCode\":401,",
        "\"statusCode\":402,",
    ];

    let re = Regex::new(r"^.*reqId.:.(.{36}).*$").unwrap();

    return lines
        .iter()
        .filter_map(|l| {
            if !l.contains("level\":50") {
                return None;
            }

            let contains_ignorable_error = ignorable_errors.iter().any(|e| l.contains(e));
            if contains_ignorable_error {
                return None;
            }

            let Some(captures) = re.captures(l) else { return None; };
            let Some(id) = captures.get(1) else { return None; };
            return Some(id.as_str());
        })
        .collect::<HashSet<_>>();
}

fn group_by_request_id<'a>(
    lines: &'a Vec<&'a str>,
    relevant_requests_ids: &'a HashSet<&'a str>,
) -> HashMap<String, Vec<&'a str>> {
    println!("grouping by request ID...");

    let mut groups: HashMap<String, Vec<&str>> = HashMap::new();
    let re = Regex::new(r"^.*reqId.:.(.{36}).*$").unwrap();

    let mut prog = Progress::new(lines.len());

    for line in lines {
        prog.print_and_increment();

        let Some(captures) =  re.captures(line) else { continue; };
        let Some(id) = captures.get(1) else { continue; };
        let id = id.as_str();

        if !relevant_requests_ids.contains(&id) {
            continue;
        }

        match groups.get_mut(id) {
            Some(group) => group.push(line),
            None => drop(groups.insert(id.to_string(), vec![line])),
        };
    }

    return groups;
}

fn get_unique_error_messages<'a>(
    groups: &'a HashMap<String, Vec<&'a str>>,
) -> (HashMap<&'a str, i32>, HashMap<&String, Vec<&'a str>>) {
    println!("getting unique errors...");

    let mut unique_errors = HashMap::new();
    let mut unknown_errors = HashMap::new();

    for (request_id, group) in groups.iter() {
        let Some(msg) = get_group_error_message(group) else {
            unknown_errors.insert(request_id, group.clone());
            continue;
        };

        let count = unique_errors.get(msg).unwrap_or(&0);
        unique_errors.insert(msg, count + 1);
    }

    return (unique_errors, unknown_errors);
}

fn get_group_error_message<'a>(group: &'a Vec<&'a str>) -> Option<&'a str> {
    let re = Regex::new(r"^.*message.:.(.*?).,.stack.*$").unwrap();

    for line in group {
        let group_message = re.captures(line).map(|caps| caps.get(1).unwrap().as_str());

        if group_message.is_some() {
            return group_message;
        }
    }

    return None;
}
