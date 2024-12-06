use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> io::Result<Vec<String>> {
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
    let count_rows = rows.len();
    let mut columns: Vec<String> = Vec::new();
    for i in 0..count_rows {
        let mut column = String::new();
        for j in 0..count_rows {
            column.push(rows[j].chars().nth(i).unwrap());
        }
        columns.push(column);
    } // columns
    let max_idex = count_rows - 1;
    let mut diagonals: Vec<String> = Vec::new();
    for i in 0..count_rows { // diagonals from left down to right up
        let mut cross = String::new();
        for j in 0..=i {
            cross.push(rows[i-j].chars().nth(j).unwrap());
        }
        diagonals.push(cross);
    }
    for i in (0..max_idex).rev() {
        let mut cross = String::new();
        for j in 0..=i {
            cross.push(rows[max_idex-j].chars().nth(max_idex-i+j).unwrap());
        }
        diagonals.push(cross);
    }
    for i in 0..count_rows { // diagonals from left up to right down
        let mut cross = String::new();
        for j in 0..=i {
            cross.push(rows[j].chars().nth(max_idex-i+j).unwrap());
        }
        diagonals.push(cross);
    }
    for i in (0..max_idex).rev() {
        let mut cross = String::new();
        for j in 0..=i {
            cross.push(rows[max_idex-i+j].chars().nth(j).unwrap());
        }
        diagonals.push(cross);
    }
    let regex1 = Regex::new(r"XMAS").unwrap();
    let regex2 = Regex::new(r"SAMX").unwrap();
    let mut result = 0;
    for i in 0..rows.len() {
        for cap in regex1.captures_iter(&rows[i]) {
            result += 1;
        }
        for cap in regex2.captures_iter(&rows[i]) {
            result += 1;
        }
    }
    for i in 0..columns.len() {
        for cap in regex1.captures_iter(&columns[i]) {
            result += 1;
        }
        for cap in regex2.captures_iter(&columns[i]) {
            result += 1;
        }
    }
    for i in 0..diagonals.len() {
        for cap in regex1.captures_iter(&diagonals[i]) {
            result += 1;
        }
        for cap in regex2.captures_iter(&diagonals[i]) {
            result += 1;
        }
    }
    println!("Part 1 result: {}", result);
}
fn main() {
    part1();
}