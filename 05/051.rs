use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("File Not Exists");
    let mut reader = BufReader::new(file);

    let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut test_vec = Vec::new();

    let mut flag = 1;
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() == 0 || parts[0] == "" {
            flag = 2;
        }

        if flag == 1 {
            if let (Some(item1), Some(item2)) = (parts.get(0), parts.get(1)) {
                if let (Ok(temp1), Ok(temp2)) = (item1.parse::<i32>(), item2.parse::<i32>()) {
                    if let Some(cur_vec) = hash_map.get(&temp1) {
                        let mut cur_vec = cur_vec.clone();
                        cur_vec.push(temp2);
                        hash_map.insert(temp1, cur_vec);
                    } else {
                        let mut new_vec = Vec::new();
                        new_vec.push(temp2);
                        hash_map.insert(temp1, new_vec);
                    }
                }
            }
        }

        if flag == 2 {
            let parts: Vec<&str> = parts[0].split(',').collect();
            for i in parts {
                if let Ok(num) = i.parse::<i32>() {
                    test_vec.push(num);
                }
            }
            sum += exercise(&test_vec, &hash_map);
            test_vec.clear();
        }
    }
    println!("{sum}");
    Ok(())
}

fn exercise(vec: &Vec<i32>, hash_map: &HashMap<i32, Vec<i32>>) -> i32 {
    if vec.len() == 0 {
        return 0;
    }
    let mut valid = true;
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            let cur_vec = hash_map.get(&vec[i]).expect("Doens exists");
            let cmp_val = vec[j];
            if !cur_vec.contains(&cmp_val) {
                valid = false;
            }
        }
    }

    if valid {
        return vec[vec.len() / 2];
    }
    return 0;
}
