use std::fs::File;

fn parse_puzzle() -> Result<Vec<Vec<i32>>, std::io::Error> {
    let mut file = File::open("./puzzle.txt")?;
    let mut contents = String::new();

    use std::io::Read;
    file.read_to_string(&mut contents)?;

    let mut lists: Vec<Vec<i32>> = vec![];

    for line in contents.split("\n") {
        lists.push(
            line.split_whitespace()
                .map(|item| item.trim().parse().expect("Failed to parse Integer"))
                .collect(),
        )
    }

    return Ok(lists);
}

fn is_list_safe(list: &Vec<i32>, tolerance: i32) -> bool {
    let mut count_unsafe: i32 = 0;
    let is_asc: bool = list[0] > list[1];

    for i in 0..(list.len() - 1) {
        let diff: i32 = list[i] - list[i + 1];

        if diff.abs() > 3 || diff == 0 || (is_asc && diff < 0) || (!is_asc && diff > 0) {
            count_unsafe += 1;

            if count_unsafe > tolerance {
                return false;
            }
        }
    }

    return true;
}

fn part1(puzzle: &Vec<Vec<i32>>) {
    let mut safe_list_count: i32 = 0;

    for list in puzzle {
        if is_list_safe(list, 0) {
            safe_list_count += 1;
        }
    }

    println!("Part 1 result: {}", safe_list_count);
}

fn part2(puzzle: &Vec<Vec<i32>>) {
    let mut safe_list_count: i32 = 0;

    for list in puzzle {
        if is_list_safe(list, 1) {
            safe_list_count += 1;
        }
    }

    println!("Part 2 result: {}", safe_list_count);
}

fn main() {
    let puzzle: Vec<Vec<i32>> = match parse_puzzle() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            return;
        }
    };

    part1(&puzzle);
    part2(&puzzle);
}
