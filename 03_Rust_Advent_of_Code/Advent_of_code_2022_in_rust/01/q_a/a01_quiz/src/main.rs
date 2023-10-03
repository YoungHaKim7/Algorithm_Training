use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct FileInput {
    data: i32,
}

struct FileOpen(fs::File);

impl FileOpen {
    fn new(file: fs::File) -> Self {
        Self(file)
    }
}

impl FileInput {
    fn max_check(&self, reader: &mut dyn BufRead) -> i32 {
        let mut current = 0;
        let mut max = 0;

        for line in reader.lines() {
            let line = line.expect("error");

            if line.is_empty() {
                if current > max {
                    max = current;
                }
                current = 0;
                continue;
            }

            match line.parse::<i32>() {
                Ok(n) => {
                    current += n;
                }
                Err(e) => {
                    println!("Error: {e}");
                    continue;
                }
            };
        }
        max
    }

    fn top_three(&self, reader02: &mut dyn BufRead) -> i32 {
        let mut elves = Vec::new();

        let mut current = 0;

        for line in reader02.lines() {
            let line = line.expect("error");

            // If line is empty, save the elf
            if line.is_empty() {
                elves.push(current);
                current = 0;
                continue;
            }

            match line.parse::<i32>() {
                Ok(n) => {
                    current += n;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            };
        }

        // Sort the elves vector in descending order
        elves.sort_by(|a, b| b.cmp(a));

        // New vector with just the top three:
        let top_three = &elves[0..3];

        // // Print out the top three
        let mut sum = 0;
        for elf in top_three {
            println!("{}", elf);
            sum += elf;
        }
        sum
    }
}

impl Iterator for FileInput {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this method to read the next line from the reader and parse it as an i32.
        // If the end of the file is reached, return None.

        None
    }
}

fn main() -> io::Result<()> {
    // Make sure that the file exists and that you have permission to read it.
    let file = fs::File::open("../quiz.txt")?;
    let file02 = fs::File::open("../quiz.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut reader02 = io::BufReader::new(file02);

    // Create a FileInput object from the reader object.
    let file_input = FileInput { data: 0 };
    let file_input2 = FileInput { data: 0 };

    // Call the max_check() method on the FileInput object.
    let max = file_input.max_check(&mut reader);
    let sum = file_input2.top_three(&mut reader02);

    // Print the max value.
    println!("Max: {}", max);

    // Print the sum value.
    println!("Elf count top 3 sum = {}", sum);

    Ok(())
}
