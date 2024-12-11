use std::io::{ BufRead, BufReader, Result};
use std::fs::File;
use std::collections::HashSet;

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
    let mut rows = read_file("src/input.txt").unwrap();
    let mut beginning = (0, 0);
    for i in 0..rows.len() {
        if rows[i].contains("^") {
            beginning = (i, rows[i].find("^").unwrap());
            break;
        }
    }
    let max_direction = rows.len() - 1;
    let mut direction = 0; //0 = up, 1 = right, 2 = down, 3 = left
    let mut row = beginning.0;
    let mut col = beginning.1;
    loop {
        rows[row].replace_range(col..col+1, "x");
        match direction {
            0 => {
                if row == 0 { break; }
                if rows[row-1].chars().nth(col).unwrap() == '#' {
                    if rows[row].chars().nth(col+1).unwrap() == '#' {
                        direction = 2;
                        row += 1;
                    } else {
                        direction = 1;
                        col += 1;
                    }
                }
                else {
                    row -= 1;
                }
            }
            1 => {
                if col + 1 > max_direction { break; }
                if rows[row].chars().nth(col+1).unwrap() == '#' {
                    if rows[row+1].chars().nth(col).unwrap() == '#' {
                        direction = 3;
                        col -= 1;
                    } else {
                        direction = 2;
                        row += 1;
                    }
                }
                else {
                    col += 1;
                }
            }
            2 => {
                if row + 1 > max_direction { break; }
                if rows[row+1].chars().nth(col).unwrap() == '#' {
                    if rows[row].chars().nth(col-1).unwrap() == '#' {
                        direction = 0;
                        row -= 1;
                    } else {
                        direction = 3;
                        col -= 1;
                    }
                }
                else {
                    row += 1;
                }
            }
            3 => {
                if col == 0 { break; }
                if rows[row].chars().nth(col-1).unwrap() == '#' {
                    if rows[row-1].chars().nth(col).unwrap() == '#' {
                        direction = 1;
                        col += 1;
                    } else {
                        direction = 0;
                        row -= 1;
                    }
                }
                else {
                    col -= 1;
                }
            }
            _ => {}
        };
    }
    let mut result = 0;
    for i in 0..rows.len() {
        result += rows[i].chars().filter(|c| *c == 'x').count();
    }
    println!("Part 1 result: {}", result);
}
fn try_loop(rows: &mut Vec<String>, beginning: (usize, usize), mut direction: usize) -> bool {
    let mut row = beginning.0;
    let mut col = beginning.1;
    let max_direction = rows.len() - 1;
    let mut count = 0;
    while count < 10000 {
        count += 1;
        match direction {
            0 => {
                if row == 0 { return false; }
                if rows[row-1].chars().nth(col).unwrap() == '#' {
                    if rows[row].chars().nth(col+1).unwrap() == '#' {
                        direction = 2;
                        row += 1;
                    } else {
                        direction = 1;
                        col += 1;
                    }
                }
                else {
                    row -= 1;
                }
            }
            1 => {
                if col + 1 > max_direction { return false; }
                if rows[row].chars().nth(col+1).unwrap() == '#' {
                    if rows[row+1].chars().nth(col).unwrap() == '#' {
                        direction = 3;
                        col -= 1;
                    } else {
                        direction = 2;
                        row += 1;
                    }
                }
                else {
                    col += 1;
                }
            }
            2 => {
                if row + 1 > max_direction { return false; }
                if rows[row+1].chars().nth(col).unwrap() == '#' {
                    if rows[row].chars().nth(col-1).unwrap() == '#' {
                        direction = 0;
                        row -= 1;
                    } else {
                        direction = 3;
                        col -= 1;
                    }
                }
                else {
                    row += 1;
                }
            }
            3 => {
                if col == 0 { return false; }
                if rows[row].chars().nth(col-1).unwrap() == '#' {
                    if rows[row-1].chars().nth(col).unwrap() == '#' {
                        direction = 1;
                        col += 1;
                    } else {
                        direction = 0;
                        row -= 1;
                    }
                }
                else {
                    col -= 1;
                }
            }
            _ => {}
        };
    }
    true
}
fn draw(rows: &mut Vec<String>) {
    for i in 0..rows.len() {
        println!("{}", rows[i]);
    }
}
fn part2() {
    let mut rows = read_file("src/input.txt").unwrap();
    let mut beginning = (0, 0);
    for i in 0..rows.len() {
        if rows[i].contains("^") {
            beginning = (i, rows[i].find("^").unwrap());
            break;
        }
    }
    let max_direction = rows.len() - 1;
    let mut direction = 0; //0 = up, 1 = right, 2 = down, 3 = left
    let mut row = beginning.0;
    let mut col = beginning.1;
    let mut result = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        visited.insert((row, col));
        rows[row].replace_range(col..col+1, "x");
        match direction {
            0 => {
                if row == 0 { break; }
                if rows[row-1].chars().nth(col).unwrap() == '#' {
                    if rows[row].chars().nth(col+1).unwrap() == '#' {
                        direction = 2;
                        row += 1;
                    } else {
                        direction = 1;
                        col += 1;
                    }
                }
                else {
                    row -= 1;
                }
            }
            1 => {
                if col + 1 > max_direction { break; }
                if rows[row].chars().nth(col+1).unwrap() == '#' {
                    if rows[row+1].chars().nth(col).unwrap() == '#' {
                        direction = 3;
                        col -= 1;
                    } else {
                        direction = 2;
                        row += 1;
                    }
                }
                else {
                    col += 1;
                }
            }
            2 => {
                if row + 1 > max_direction { break; }
                if rows[row+1].chars().nth(col).unwrap() == '#' {
                    if rows[row].chars().nth(col-1).unwrap() == '#' {
                        direction = 0;
                        row -= 1;
                    } else {
                        direction = 3;
                        col -= 1;
                    }
                }
                else {
                    row += 1;
                }
            }
            3 => {
                if col == 0 { break; }
                if rows[row].chars().nth(col-1).unwrap() == '#' {
                    if rows[row-1].chars().nth(col).unwrap() == '#' {
                        direction = 1;
                        col += 1;
                    } else {
                        direction = 0;
                        row -= 1;
                    }
                }
                else {
                    col -= 1;
                }
            }
            _ => {}
        };
    }
    for (i, j) in visited {
        rows[i].replace_range(j..j+1, "#");
        if try_loop(&mut rows, beginning, 0) {
            result += 1;
        }
        rows[i].replace_range(j..j+1, ".");
    }
    println!("Part 2 result: {}", result);
}
fn main() {
    part1();
    part2();
}
