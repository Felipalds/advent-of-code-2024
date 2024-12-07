use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("File not exists");
    let mut reader = BufReader::new(file);

    let mut matrix: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line);
    }

    // let stri = String::from("Ola rodrigues");
    // rust does not allows like stri[1]
    // because chars are UTF-8 and not ascii
    // one char can have multiple bytes

    let mut xmas = 0;
    let mut mas = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch == 'X' {
                xmas += verify_xmas(i, j, &matrix);
            }

            if ch == 'A' {
                mas += verify_mas(i, j, &matrix);
            }
        }
    }

    println!("{xmas}, {mas}");
    Ok(())
}

fn verify_mas(i: usize, j: usize, m: &Vec<String>) -> i32 {
    let mut valid = 0;
    println!("Testing");
    if i + 1 < m.len() && j + 1 < m[i].len() && i >= 1 && j >= 1 {
        if let Some(ch1) = m[i + 1].chars().nth(j + 1) {
            if let Some(ch2) = m[i - 1].chars().nth(j - 1) {
                println!("{ch1}, {ch2}");
                if (ch1 == 'M' && ch2 == 'S') || (ch1 == 'S' && ch2 == 'M') {
                    valid = 1;
                }
            }
        }

        if valid == 0 {
            return 0;
        }

        if let Some(ch1) = m[i + 1].chars().nth(j - 1) {
            if let Some(ch2) = m[i - 1].chars().nth(j + 1) {
                println!("{ch1}, {ch2}");
                if (ch1 == 'M' && ch2 == 'S') || (ch1 == 'S' && ch2 == 'M') {
                    valid = 2;
                }
            }
        }
    }

    if valid == 2 {
        1
    } else {
        0
    }
}

fn verify_xmas(i: usize, j: usize, m: &Vec<String>) -> i32 {
    let mut xmas = 0;
    let mut buf = String::from("");

    // right
    if j + 4 <= m[i].len() {
        for k in 0..4 {
            if let Some(ch) = m[i].chars().nth(j + k) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // left
    if j >= 3 {
        for k in 0..4 {
            if let Some(ch) = m[i].chars().nth(j - k) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // up
    if i >= 3 {
        for k in 0..4 {
            if let Some(ch) = m[i - k].chars().nth(j) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // down
    if i + 4 <= m.len() {
        for k in 0..4 {
            if let Some(ch) = m[i + k].chars().nth(j) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // diagonal left upper
    if j >= 3 && i >= 3 {
        for k in 0..4 {
            if let Some(ch) = m[i - k].chars().nth(j - k) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // diagonal right upper
    if j + 4 <= m.len() && i >= 3 {
        for k in 0..4 {
            if let Some(ch) = m[i - k].chars().nth(j + k) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // diagonal right down
    if j + 4 <= m.len() && i + 4 <= m.len() {
        for k in 0..4 {
            if let Some(ch) = m[i + k].chars().nth(j + k) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    // diagonal left down
    if j >= 3 && i + 4 <= m.len() {
        for k in 0..4 {
            if let Some(ch) = m[i + k].chars().nth(j - k) {
                buf.push(ch);
            }
        }
        if buf == "XMAS" {
            xmas += 1;
        }
        buf.clear();
    }

    xmas
}
