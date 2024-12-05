use std::fs::File;

fn parse_puzzle() -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), std::io::Error> {
    let mut file = File::open("./puzzle.txt")?;
    let mut contents = String::new();

    use std::io::Read;
    file.read_to_string(&mut contents)?;

    let splitted_content: Vec<&str> = contents.split("\n\n").collect();

    let mut top: Vec<(i32, i32)> = vec![];
    let mut bottom: Vec<Vec<i32>> = vec![];

    for line in splitted_content[0].split("\n") {
        let numbers: Vec<i32> = line
            .split("|")
            .map(|num| num.trim().parse().expect("Failed to parse number"))
            .collect();
        top.push((numbers[0], numbers[1]));
    }

    for line in splitted_content[1].split("\n") {
        bottom.push(
            line.split(",")
                .map(|num| num.trim().parse().expect("Failed to parse number"))
                .collect(),
        )
    }

    Ok((top, bottom))
}

fn part1(puzzle: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) {
    let mut total: i32 = 0;

    for line in &puzzle.1 {
        'block: {
            for (index, number) in line.iter().enumerate() {
                for rule in &puzzle.0 {
                    if rule.1 == *number && line[index..].contains(&rule.0) {
                        break 'block;
                    }
                }
            }
            total += line[((line.len() as f32) / 2.0).ceil() as usize - 1]
        }
    }

    println!("Part 1 result: {total}");
}

fn part2(puzzle: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) {
    let mut total: i32 = 0;

    for line in &puzzle.1 {
        'filter_block: {
            let mut misplaced_values = 0;
            for (index, number) in line.iter().enumerate() {
                for rule in &puzzle.0 {
                    if rule.0 == *number && line[..index].contains(&rule.1) {
                        misplaced_values += 1;
                    }
                }
            }
            if misplaced_values == 0 {
                break 'filter_block;
            }

            let mut old_line: Vec<i32> = line.clone();
            let mut new_line: Vec<i32> = vec![];

            while old_line.len() > 0 {
                let mut index_to_remove: i32 = -1;

                for (index, number) in old_line.iter().enumerate() {
                    'sorting_block: {
                        for rule in &puzzle.0 {
                            if rule.1 == *number && old_line[..index].contains(&rule.0) {
                                break 'sorting_block;
                            }
                        }
                        index_to_remove = index as i32;
                    }
                }

                if index_to_remove != -1 {
                    new_line.push(old_line.remove(index_to_remove as usize))
                }
            }

            total += new_line[((new_line.len() as f32) / 2.0).ceil() as usize - 1]
        }
    }

    println!("Part 2 result: {total}");
}

fn main() {
    let puzzle: (Vec<(i32, i32)>, Vec<Vec<i32>>) = match parse_puzzle() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            return;
        }
    };

    part1(&puzzle);
    part2(&puzzle);
}
