use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Open a file for reading
    let mut file = File::open("input.txt")?;

    // Read the contents of the file into a string
    let reader = io::BufReader::new(file);
    let mut result = 0;
    // Iterate over lines in the file
    for line in reader.lines() {
        let current_line = line?;
        let mut num_str = String::new();
        for ch in current_line.chars() {
            if ch.is_digit(10) {
                if !num_str.is_empty() {
                    num_str.pop();
                } else {
                    num_str.push(ch);
                }
                num_str.push(ch);
            }
        }
        if num_str.len() == 1 {
            num_str.push(num_str.chars().next().expect("String is empty"));
        }

        let num: Result<i32, _> = num_str.parse();
        match num {
            Ok(parsed_int) => {
                println!("Number found: {}", parsed_int);
                result += parsed_int;
            }
            Err(err) => {
                eprintln!("Error parsing integer: {}", err);
            }
        }
    }
    println!("Result {}", result);
    Ok(())
}
