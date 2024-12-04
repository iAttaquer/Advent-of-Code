use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn read_file(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut rows: Vec<Vec<i32>> = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        let parts = line.split_whitespace();
        let mut row: Vec<i32> = Vec::new();
        for i in parts {
            row.push(i.parse::<i32>().unwrap());
        }
        rows.push(row);
    }
    Ok(rows)
}
fn is_safe(row: &mut Vec::<i32>) -> bool {
    let mut increasing = 0; // 0 : none, 1 : increasing, 2 : decreasing
    for i in 0..row.len() - 1 {
        let difference = row[i+1] - row[i];
        if difference > 0 && difference.abs() <= 3 {
            if increasing == 2 {
                return false;
            }
            increasing = 1;
        } else if difference < 0 && difference.abs() <= 3 {
            if increasing == 1 {
                return false;
            }
            increasing = 2;
        }
        else {
            return false;
        }
    }
    true
}
fn part1() {
    let mut matrix = read_file("src/input.txt").unwrap();
    let mut result = 0;
    for i in 0..matrix.len() {
        if is_safe(&mut matrix[i]) {
            result += 1;
        }
    }
    println!("Part 1 result: {}", result);
}
fn part2() {
    let mut matrix = read_file("src/input.txt").unwrap();
    let mut result = 0;
    for i in 0..matrix.len() {
        if is_safe(&mut matrix[i]) {
            result += 1;
            continue;
        }
        for j in 0..matrix[i].len() {
            let mut new_row = matrix[i].clone();
            new_row.remove(j);
            if is_safe(&mut new_row) {
                result += 1;
                break;
            }
        }
    }
    println!("Part 2 result: {}", result);
}

fn main() {
    part1();
    part2();
}
