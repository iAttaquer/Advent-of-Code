use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};

fn read_file(path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
        let mut parts = line.split_whitespace().take(2);
        match (parts.next(), parts.next()) {
            (Some(n1), Some(n2)) => {
                list1.push(n1.parse::<i32>().unwrap());
                list2.push(n2.parse::<i32>().unwrap());
            }
            _ => {
                eprintln!("Skipped line");
            }
        }
    }
    Ok((list1, list2))
}
fn part1() -> io::Result<()> {
    let (mut list1, mut list2) = read_file("src/input.txt")?;
    list1.sort();
    list2.sort();
    let mut result = 0;
    for i in 0..list1.len() {
        result += (list2[i] - list1[i]).abs();
    }
    println!("Part 1 result: {}", result);
    Ok(())
}
fn count_occurence(list: &Vec<i32>, element: i32) -> i32 {
    let mut count = 0;
    for item in list {
        if *item == element {
            count += 1;
        }
    }
    count
}
fn part2() -> io::Result<()> {
    let (list1, list2) = read_file("src/input.txt")?;
    let mut result = 0;
    for i in 0..list1.len() {
        result += list1[i] * count_occurence(&list2, list1[i]);
    }
    println!("Part 2 result: {}", result);
    Ok(())
}
fn main() {
    part1();
    part2();
}
