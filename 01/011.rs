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

    let mut dist: i32 = 0;

    for (index, list1val) in list1.iter().enumerate() {
        let list2val = match list2.get(index) {
            Some(&val) => val,
            None => {
                break;
            }
        };

        if *list1val > list2val {
            dist += *list1val - list2val;
        } else {
            dist += list2val - *list1val;
        }
    }

    println!("{}", dist);

    Ok(())
}
