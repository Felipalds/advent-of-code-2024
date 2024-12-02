use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(&path)?;
    let mut valid = 0;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut list = Vec::new();
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        for i in 0..parts.len() {
            if let Some(item) = parts.get(i) {
                if let Ok(num) = item.parse::<i32>() {
                    list.push(num);
                }
            }
        }
        let ivalid = is_valid(&list);
        if ivalid {
            valid += 1
        } else {
            for i in 0..list.len() {
                let mut clone_list = list.clone();
                clone_list.remove(i);
                if is_valid(&clone_list) {
                    valid += 1;
                    break;
                }
            }
        }
        println!("{}", valid)
    }

    Ok(())
}

fn is_valid(list: &[i32]) -> bool {
    let mut prev = list[0];
    let mut order: &str = "NIL";
    let mut isValid = true;

    for i in 1..list.len() {
        let current = list[i];
        let sub = (current - prev).abs();
        if sub > 3 || sub < 1 {
            isValid = false;
            break;
        }
        if order == "NIL" {
            if list[i] > prev {
                order = "ASC";
            } else {
                order = "DESC";
            }
            prev = list[i];
            continue;
        }
        if order == "ASC" {
            if current < prev {
                isValid = false;
                break;
            }
        }
        if order == "DESC" {
            if current > prev {
                isValid = false;
                break;
            }
        }
        prev = current;
    }
    println!("{:?}", list);
    return isValid;
}
