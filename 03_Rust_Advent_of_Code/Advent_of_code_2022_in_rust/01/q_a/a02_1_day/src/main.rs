use std::fs;
use std::path::Path;

fn read_data(data_path: &std::path::Path) -> String {
    fs::read_to_string(&data_path).expect("data read fail")
}

fn part_1() -> i32 {
    let mut data = read_data(Path::new("../quiz.txt"));
    data.push('\n');
    let mut first = 0;

    let mut buff = 0;

    for line in data.lines() {
        if line.is_empty() {
            if buff > first {
                first = buff;
            }
            buff = 0;
        } else {
            buff += line.parse::<i32>().expect("not a num");
        }
    }
    first
}

fn part_2() -> i32 {
    let mut data = read_data(Path::new("../quiz.txt"));
    data.push('\n');
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    let mut buff = 0;
    for line in data.lines() {
        if line.is_empty() {
            if buff > first {
                third = second;
                second = first;
                first = buff;
            } else if buff > second {
                third = second;
                second = buff;
            } else if buff > third {
                third = buff;
            }

            buff = 0;
        } else {
            buff += line.parse::<i32>().expect("not number");
        }
    }
    first + second + third
}

pub fn main() {
    // println!("{}", part_1());
    println!("{}", part_2());
}
