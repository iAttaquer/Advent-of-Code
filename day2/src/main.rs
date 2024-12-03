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
fn part1() {
    let matrix = read_file("src/input.txt").unwrap();
    let mut result = 0;
    for i in 0..matrix.len() {
        let first_difference = matrix[i][1] - matrix[i][0];
        let increasing: bool =
            if first_difference > 0 {
                if first_difference.abs() <= 3 {
                    true
                } else {
                    continue;
                }
            }
            else if first_difference < 0 {
                if first_difference.abs() <= 3 {
                    false
                } else {
                    continue;
                }
            }
            else { continue };
        for j in 1..matrix[i].len() - 1 {
            if increasing == true {
                let x = matrix[i][j+1] - matrix[i][j];
                if x > 0 {
                    if x.abs() <= 3 {
                        if j == matrix[i].len() - 2 {
                            result += 1;
                        }
                    }
                    else {
                        break;
                    }
                } else {
                    break;
                    }
                }
            else if increasing == false {
                let x = matrix[i][j+1] - matrix[i][j];
                if x < 0 {
                    if x.abs() <= 3 {
                        if j == matrix[i].len() - 2 {
                            result += 1;
                        }
                    } else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        }
    }
    println!("Part 1 result: {}", result);
}
fn main() {
    part1();
}
