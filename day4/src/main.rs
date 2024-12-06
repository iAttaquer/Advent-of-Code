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
    // columns
    for i in 0..count_rows {
        let mut column = String::new();
        for j in 0..count_rows {
            column.push(rows[j].chars().nth(i).unwrap());
        }
        columns.push(column);
    }
    let max_idex = count_rows - 1;
    let mut diagonals: Vec<String> = Vec::new();
    // diagonals from left down to right up
    for i in 0..count_rows {
        let mut diagonal = String::new();
        for j in 0..=i {
            diagonal.push(rows[i-j].chars().nth(j).unwrap());
        }
        diagonals.push(diagonal);
    }
    for i in (0..max_idex).rev() {
        let mut diagonal = String::new();
        for j in 0..=i {
            diagonal.push(rows[max_idex-j].chars().nth(max_idex-i+j).unwrap());
        }
        diagonals.push(diagonal);
    }
    // diagonals from left up to right down
    for i in 0..count_rows {
        let mut diagonal = String::new();
        for j in 0..=i {
            diagonal.push(rows[j].chars().nth(max_idex-i+j).unwrap());
        }
        diagonals.push(diagonal);
    }
    for i in (0..max_idex).rev() {
        let mut diagonal = String::new();
        for j in 0..=i {
            diagonal.push(rows[max_idex-i+j].chars().nth(j).unwrap());
        }
        diagonals.push(diagonal);
    }
    let regex1 = Regex::new(r"XMAS").unwrap();
    let regex2 = Regex::new(r"SAMX").unwrap();
    let mut result = 0;
    for i in 0..rows.len() {
        result += regex1.captures_iter(&rows[i]).count();
        result += regex2.captures_iter(&rows[i]).count();
    }
    for i in 0..columns.len() {
        result += regex1.captures_iter(&columns[i]).count();
        result += regex2.captures_iter(&columns[i]).count();
    }
    for i in 0..diagonals.len() {
        result += regex1.captures_iter(&diagonals[i]).count();
        result += regex2.captures_iter(&diagonals[i]).count();
    }
    println!("Part 1 result: {}", result);
}
fn part2() {
    let rows = read_file("src/input.txt").unwrap();
    let mut result = 0;
    for i in 1..rows.len() - 1 {
        for j in 1..rows[0].len() - 1 {
            if rows[i].chars().nth(j).unwrap() == 'A'
            && rows[i-1].chars().nth(j-1).unwrap() == 'M'
            && rows[i-1].chars().nth(j+1).unwrap() == 'M'
            && rows[i+1].chars().nth(j-1).unwrap() == 'S'
            && rows[i+1].chars().nth(j+1).unwrap() == 'S' {
                result += 1;
            }
            if rows[i].chars().nth(j).unwrap() == 'A'
            && rows[i-1].chars().nth(j-1).unwrap() == 'S'
            && rows[i-1].chars().nth(j+1).unwrap() == 'M'
            && rows[i+1].chars().nth(j-1).unwrap() == 'S'
            && rows[i+1].chars().nth(j+1).unwrap() == 'M' {
                result += 1;
            }
            if rows[i].chars().nth(j).unwrap() == 'A'
            && rows[i-1].chars().nth(j-1).unwrap() == 'S'
            && rows[i-1].chars().nth(j+1).unwrap() == 'S'
            && rows[i+1].chars().nth(j-1).unwrap() == 'M'
            && rows[i+1].chars().nth(j+1).unwrap() == 'M' {
                result += 1;
            }
            if rows[i].chars().nth(j).unwrap() == 'A'
            && rows[i-1].chars().nth(j-1).unwrap() == 'M'
            && rows[i-1].chars().nth(j+1).unwrap() == 'S'
            && rows[i+1].chars().nth(j-1).unwrap() == 'M'
            && rows[i+1].chars().nth(j+1).unwrap() == 'S' {
                result += 1;
            }
        }
    }
    println!("Part 2 result: {}", result);

}
fn main() {
    part1();
    part2();
}