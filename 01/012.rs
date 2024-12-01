use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";
    let file = File::open(&path)?;

    // here we are creating two lists
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(first), Some(second)) = (parts.get(0), parts.get(1)) {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    list1.sort();
    list2.sort();

    let mut sim: i32 = 0;
    let mut index2 = 0;

    for (_index, list1val) in list1.iter().enumerate() {
        while index2 < list2.len() && list2[index2] < *list1val {
            index2 += 1;
        }

        let mut count = 0;

        while index2 < list2.len() && list2[index2] == *list1val {
            count += 1;
            index2 += 1;
        }

        sim += *list1val * count as i32;
    }

    println!("{}", sim);

    Ok(())
}
