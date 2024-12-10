use std::io::{BufRead, BufReader, Result};
use std::fs::File;
use std::collections::HashMap;

fn read_file(path: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut rows: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        rows.push(line);
    }
    Ok(rows)
}
fn part1() {
    let rows = read_file("src/input.txt").unwrap();
    let mut rules: Vec<String> = Vec::new();
    let mut queues: Vec<String> = Vec::new();
    let mut is_rules = true;
    for i in 0..rows.len() {
        if rows[i].len() == 0 {
            is_rules = false;
            continue;
        }
        if is_rules {
            rules.push(rows[i].clone());
        }
        else {
            queues.push(rows[i].clone());
        }
    }
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for i in 0..rules.len() {
        let first = &rules[i][..2];
        let second = &rules[i][3..];
        map.entry(second.to_string()).or_insert(Vec::new()).push(first.to_string());
    }
    let mut result = 0;
    for i in 0..queues.len() {
        let mut queue = queues[i].split(',').collect::<Vec<&str>>();
        let mut correct = true;
        for j in 0..queue.len() - 1 {
            if let Some(v) = map.get(&queue[j].to_string()) {
                for k in j+1..queue.len() {
                    if map.get(&queue[j].to_string()).unwrap().contains(&queue[k].to_string()) {
                        correct = false;
                    }
                }
            }
        }
        if correct {
            result += queue[queue.len() / 2].parse::<i32>().unwrap();
        }
    }
    println!("Part 1 result: {}", result);
}
fn main() {
    part1();
}
