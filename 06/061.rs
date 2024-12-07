use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(PartialEq)]

enum Directions {
    Up,
    Left,
    Right,
    Down
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("");
    let reader = BufReader::new(file);
    let mut dir: Directions = Directions::Up;

    let mut matrix: Vec<Vec<char>> = Vec::new();
    // let mut route: Vec<Vec<char>> = Vec::new();
    let mut pos = 0;

    for line in reader.lines() {
        let line = line?;
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }

    // route = matrix.clone();
    let mut x: usize = 0;
    let mut y: usize = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                x = i;
                y = j;
            }
        }
    }

    loop {
        if matrix[x][y] != 'X' {
            pos += 1;
            matrix[x][y] = 'X';
        }
        if dir == Directions::Up && x == 0 {
            break;
        }
        if dir == Directions::Down && x == matrix.len() - 1 {
            break;
        }
        if dir == Directions::Left && y == 0 {
            break;
        }
        if dir == Directions::Right && y ==  matrix[0].len() - 1 {
            break;
        }


        // route[x][y] = '#';

        if dir == Directions::Up && matrix[x-1][y] == '#' {
            dir = Directions::Right;
        }
        if dir == Directions::Right && matrix[x][y+1] == '#' {
            dir = Directions::Down
        }
        if dir == Directions::Down  && matrix[x+1][y] == '#' {
            dir = Directions::Left;
        }
        if dir == Directions::Left && matrix[x][y-1] == '#' {
            dir = Directions::Up;
        }

        match dir {
            Directions::Up => x-=1,
            Directions::Right => y+=1,
            Directions::Down => x+=1 ,
            Directions::Left => y-=1,
        }
    }
        println!("{x}, {y}, {pos}");
    // for row in &matrix {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     // Add a newline after each row if desired
    //     println!();
    Ok(())
}
