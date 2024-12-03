use std::io::{self, BufRead};
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()>{
    let file = File::open("src/input.txt")?;
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
                println!("Skipped line");
            }
        }
    }
    list1.sort();
    list2.sort();
    let mut result = 0;
    for i in 0..list1.len() {
        result += (list2[i] - list1[i]).abs();
    }
    println!("{:?}", list1);
    println!("{:?}", list2);
    println!("Result: {}", result);
    Ok(())
}
