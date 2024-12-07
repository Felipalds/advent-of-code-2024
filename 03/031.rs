use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("File opened"); // this calls the syscall to open a file
    let mut reader = BufReader::new(file); // buffer that will read the file

    let mut buf = Vec::<u8>::new(); // unsigned int and/or a byte!

    #[derive(Debug)]
    enum States {
        State_Start,
        State_Str,
        State_First_Num,
        State_Last_Num,
        State_Do,
        State_Dont,
    }

    let mut state: States = States::State_Start;
    let mut dontvalid = 0;
    // A Vec<u8> is used because read_until works with raw bytes, not characters or text.
    // After reading, the Vec<u8> is converted into a String to work with UTF-8 text.
    // This separation ensures safety and flexibility when handling potentially non-textual or binary data.

    let mut sum = 0;
    let mut flag = true;
    while reader.read_until(b'\n', &mut buf)? != 0 {
        let s = String::from_utf8(buf).expect("");
        let mut str_buff = Vec::<u8>::new();
        let mut first_num = Vec::<u8>::new();
        let mut last_num = Vec::<u8>::new();

        for c in s.chars() {
            println!("{:?}", state);
            match state {
                States::State_Start => {
                    if c.is_alphabetic() {
                        state = States::State_Str;
                        str_buff.push(c as u8);
                    } else {
                        // clean all buffers
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear()
                    }
                }
                States::State_Str => {
                    if c.is_alphabetic() {
                        state = States::State_Str;
                        str_buff.push(c as u8);

                        continue;
                    } else if c.is_digit(10) {
                        state = States::State_Start;
                        // clean all the buffers
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();
                        continue;
                    } else if c == '(' {
                        // verify if word is "mul"
                        if str_buff == b"mul" {
                            state = States::State_First_Num;
                            continue;
                        } else if &str_buff[str_buff.len() - 2..] == b"do" {
                            state = States::State_Do;
                            continue;
                        } else {
                            state = States::State_Start;
                            str_buff.clear();
                            first_num.clear();
                            last_num.clear();
                            // clean all buffers
                            continue;
                        }
                    } else if c == '\'' {
                        if str_buff == b"don" {
                            state = States::State_Dont;
                            continue;
                        }
                    } else {
                        state = States::State_Start;
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();
                        continue;
                    }
                }
                States::State_Do => {
                    if c == ')' {
                        flag = true;
                    }
                    state = States::State_Start;
                    str_buff.clear();
                    first_num.clear();
                    last_num.clear();
                    continue;
                }
                States::State_Dont => {
                    println!("Jonied");
                    if c == 't' && dontvalid == 0 {
                        dontvalid = 1;
                        continue;
                    }
                    if c == '(' && dontvalid == 1 {
                        dontvalid = 2;
                        continue;
                    }
                    if c == ')' && dontvalid == 2 {
                        flag = false;
                    }
                    dontvalid = 0;
                    state = States::State_Start;
                    str_buff.clear();
                    first_num.clear();
                    last_num.clear();
                    continue;
                }
                States::State_First_Num => {
                    if c.is_digit(10) {
                        println!("{c}");
                        first_num.push(c as u8);
                        continue;
                    }
                    if c.is_alphabetic() {
                        state = States::State_Start;
                        // clean all the buffers
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();
                        continue;
                    }
                    if c == ',' {
                        state = States::State_Last_Num;
                        continue;
                    } else {
                        // clean all buffers
                        state = States::State_Start;
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();
                        continue;
                    }
                }
                States::State_Last_Num => {
                    if c.is_digit(10) {
                        println!("{c}");
                        last_num.push(c as u8);
                        continue;
                    }
                    if c.is_alphabetic() {
                        state = States::State_Start;
                        // clean all the buffers
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();

                        continue;
                    }
                    if c == ')' {
                        state = States::State_Start;
                        // calculate here
                        // clean all buffers
                        println!("{:?}", first_num);
                        println!("{:?}", last_num);
                        let num1 = String::from_utf8(first_num.clone())
                            .unwrap()
                            .parse::<u64>()
                            .unwrap();
                        let num2 = String::from_utf8(last_num.clone())
                            .unwrap()
                            .parse::<u64>()
                            .unwrap();

                        if flag {
                            sum += num1 * num2;
                        }
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();

                        continue;
                    } else {
                        state = States::State_Start;
                        // clean all buffers
                        str_buff.clear();
                        first_num.clear();
                        last_num.clear();
                        continue;
                    }
                }
            }
        }
        buf = s.into_bytes();
        buf.clear();
    }
    println!("{sum}");
    Ok(())
}
