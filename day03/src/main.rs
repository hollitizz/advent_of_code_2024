use regex::Regex;
use std::fs::File;

fn parse_puzzle() -> Result<String, std::io::Error> {
    let mut file = File::open("./puzzle.txt")?;
    let mut contents = String::new();

    use std::io::Read;
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn part1(puzzle: &String) {
    let mut total: i32 = 0;
    let mul_regex: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let number_regex: Regex = Regex::new(r"\d+").unwrap();

    for operation in mul_regex.find_iter(&puzzle) {
        let numbers: Vec<i32> = number_regex
            .find_iter(operation.as_str())
            .map(|numb| {
                numb.as_str()
                    .trim()
                    .parse::<i32>()
                    .expect("Failed to parse Integer")
            })
            .collect();

        total += numbers[0] * numbers[1];
    }

    println!("Part 1 result: {}", total);
}

fn part2(puzzle: &String) {
    let mut total: i32 = 0;

    let mut enabled: bool = true;
    let mul_regex: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let number_regex: Regex = Regex::new(r"\d+").unwrap();

    for block in puzzle.split("don't()") {
        for substring in block.split("do()") {
            if !enabled {
                enabled = true;
                continue;
            }

            for operation in mul_regex.find_iter(&substring) {
                let numbers: Vec<i32> = number_regex
                    .find_iter(operation.as_str())
                    .map(|numb| {
                        numb.as_str()
                            .trim()
                            .parse::<i32>()
                            .expect("Failed to parse Integer")
                    })
                    .collect();

                total += numbers[0] * numbers[1];
            }
        }
        enabled = false
    }

    println!("Part 2 result: {}", total);
}

fn main() {
    let puzzle: String = match parse_puzzle() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            return;
        }
    };

    part1(&puzzle);
    part2(&puzzle);
}
