use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut rows: Vec<String> = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue; }
        };
        rows.push(line);
    }
    Ok(rows)
}
fn part1() {
    let rows = read_file("src/input.txt").unwrap();
    let mul_regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let num_regex = Regex::new(r"[0-9]+").unwrap();
    let mut result = 0;
    for i in 0..rows.len() {
        for cap in mul_regex.captures_iter(&rows[i]) {
            let mut multiply = 1;
            for num in num_regex.captures_iter(&cap[0]) {
                multiply *= num[0].parse::<i32>().unwrap();
            }
            result += multiply;
        }
    }
    println!("Part 1 result: {}", result);
}
fn part2() {
    let rows = read_file("src/input.txt").unwrap();
    let expression_regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let num_regex = Regex::new(r"[0-9]+").unwrap();
    let mut result = 0;
    let mut allowed = true;
    for i in 0..rows.len() {
        for cap in expression_regex.captures_iter(&rows[i]) {
            let mut multiply = 1;
            let matched_string = &cap[0];
            if matched_string.eq("do()") {
                allowed = true;
            } else if matched_string.eq("don't()") {
                allowed = false;
            } else {
                if allowed {
                    for num in num_regex.captures_iter(&cap[0]) {
                        multiply *= num[0].parse::<i32>().unwrap();
                    }
                    result += multiply;
                }
            }
        }
    }
    println!("Part 2 result: {}", result);
}
fn main() {
    part1();
    part2();
}
