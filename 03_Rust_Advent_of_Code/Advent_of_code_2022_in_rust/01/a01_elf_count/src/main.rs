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
}

// impl fmt::Display for FileInput {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({})", self.data)
//     }
// }

fn main() -> io::Result<()> {
    let file = fs::File::open("../input01.txt")?;
    let mut reader = io::BufReader::new(file);

    // Create a FileInput object from the reader object.
    let file_input = FileInput { data: 0 };

    // Call the max_check() method on the FileInput object.
    let max = file_input.max_check(&mut reader);

    // Print the max value.
    println!("Max: {}", max);

    Ok(())
}
