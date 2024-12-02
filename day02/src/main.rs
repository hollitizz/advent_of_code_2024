use std::fs::File;

fn parse_puzzle() -> Result<Vec<Vec<i32>>, std::io::Error> {
    let mut file = File::open("./puzzle.txt")?;
    let mut contents = String::new();

    use std::io::Read;
    file.read_to_string(&mut contents)?;

    let mut lists: Vec<Vec<i32>> = vec![];

    for line in contents.split("\n") {
        lists.push(
            line.split(" ")
                .map(|item| item.trim().parse().unwrap())
                .collect(),
        )
    }

    return Ok(lists);
}

fn is_list_safe(list: Vec<i32>, tolerance: i32) -> bool {
    let mut is_safe: bool = true;
    let mut count_unsafe: i32 = 0;
    let is_asc: bool = list[0] > list[1];

    for i in 0..(list.len() - 1) {
        let diff: i32 = list[i] - list[i + 1];

        if diff.abs() > 3 || diff == 0 {
            count_unsafe += 1;

            if count_unsafe > tolerance {
                is_safe = false;
                break;
            }
        } else if (is_asc && diff < 0) || (!is_asc && diff > 0) {
            count_unsafe += 1;

            if count_unsafe > tolerance {
                is_safe = false;
                break;
            }
        }
    }

    return is_safe;
}

fn part1() {
    let mut res: i32 = 0;

    let lists: Vec<Vec<i32>> = match parse_puzzle() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    for list in lists {
        if is_list_safe(list, 0) {
            res += 1;
        }
    }

    println!("Part 1 result: {}", res);
}

fn part2() {
    let mut res: i32 = 0;

    let lists: Vec<Vec<i32>> = match parse_puzzle() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    for list in lists {
        if is_list_safe(list, 1) {
            res += 1;
        }
    }

    println!("Part 2 result: {}", res);
}

fn main() {
    part1();
    part2();
}
