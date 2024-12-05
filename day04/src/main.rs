use std::fs::File;

fn parse_puzzle() -> Result<Vec<Vec<char>>, std::io::Error> {
    let mut file = File::open("./puzzle.txt")?;
    let mut contents = String::new();

    use std::io::Read;
    file.read_to_string(&mut contents)?;

    let mut lists: Vec<Vec<char>> = vec![];

    for line in contents.split("\n") {
        lists.push(line.chars().collect())
    }

    return Ok(lists);
}

#[derive(PartialEq, Clone)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::Top,
    Direction::TopRight,
    Direction::Right,
    Direction::BottomRight,
    Direction::Bottom,
    Direction::BottomLeft,
    Direction::Left,
    Direction::TopLeft,
];

struct Coordinate {
    x: usize,
    y: usize,
}

fn get_target_pos(direction: &Direction, current_pos: &Coordinate) -> Option<Coordinate> {
    let (new_y, new_x) = match direction {
        Direction::Top if current_pos.y > 0 => (current_pos.y - 1, current_pos.x),
        Direction::TopRight if current_pos.y > 0 => (current_pos.y - 1, current_pos.x + 1),
        Direction::Right => (current_pos.y, current_pos.x + 1),
        Direction::BottomRight => (current_pos.y + 1, current_pos.x + 1),
        Direction::Bottom => (current_pos.y + 1, current_pos.x),
        Direction::BottomLeft if current_pos.x > 0 => (current_pos.y + 1, current_pos.x - 1),
        Direction::Left if current_pos.x > 0 => (current_pos.y, current_pos.x - 1),
        Direction::TopLeft if current_pos.y > 0 && current_pos.x > 0 => {
            (current_pos.y - 1, current_pos.x - 1)
        }
        _ => return None,
    };

    Some(Coordinate { x: new_x, y: new_y })
}

fn get_letter_at(
    direction: &Direction,
    current_pos: &Coordinate,
    puzzle: &Vec<Vec<char>>,
) -> Option<char> {
    if let Some(new_pos) = get_target_pos(direction, current_pos) {
        if new_pos.y >= puzzle.len() || new_pos.x >= puzzle[0].len() {
            return None;
        }
        Some(puzzle[new_pos.y][new_pos.x])
    } else {
        None
    }
}

fn look_around(letter: &char, puzzle: &Vec<Vec<char>>, pos: &Coordinate) -> Vec<Direction> {
    let mut available_directions: Vec<Direction> = vec![];

    for direction in ALL_DIRECTIONS {
        if let Some(found_letter) = get_letter_at(&direction, pos, puzzle) {
            if *letter == found_letter {
                available_directions.push(direction);
            }
        }
    }

    available_directions
}

fn look_for(
    letter: &char,
    direction: &Direction,
    pos: &Coordinate,
    puzzle: &Vec<Vec<char>>,
) -> bool {
    if let Some(found_letter) = get_letter_at(&direction, pos, puzzle) {
        *letter == found_letter
    } else {
        false
    }
}

fn part1(puzzle: &Vec<Vec<char>>) {
    let mut total: i32 = 0;
    let word: [char; 4] = ['X', 'M', 'A', 'S'];

    for (y_index, y) in puzzle.into_iter().enumerate() {
        let mut pos: Coordinate = Coordinate { x: 0, y: y_index };
        for (x_index, x) in y.iter().enumerate() {
            if *x != word[0] {
                continue;
            }

            pos.x = x_index;
            let available_directions: Vec<Direction> = look_around(&word[1], puzzle, &pos);

            for direction in available_directions {
                'is_valid_word: {
                    let mut current_pos: Coordinate;

                    if let Some(new_pos) = get_target_pos(&direction, &pos) {
                        current_pos = new_pos;
                    } else {
                        break 'is_valid_word;
                    }

                    for letter in &word[2..] {
                        if !look_for(letter, &direction, &current_pos, puzzle) {
                            break 'is_valid_word;
                        }

                        if let Some(new_pos) = get_target_pos(&direction, &current_pos) {
                            current_pos = new_pos;
                        } else {
                            break 'is_valid_word;
                        }
                    }
                    total += 1;
                }
            }
        }
    }
    println!("Part 1 result: {total}");
}

fn is_corner_next_to(a: &Direction, b: &Direction) -> bool {
    match a {
        Direction::TopLeft | Direction::BottomRight => {
            *b == Direction::TopRight || *b == Direction::BottomLeft
        }
        Direction::TopRight | Direction::BottomLeft => {
            *b == Direction::TopLeft || *b == Direction::BottomRight
        }
        _ => false,
    }
}

fn keep_corners(directions: &Vec<Direction>) -> Vec<Direction> {
    directions
        .iter()
        .filter(|direction| match direction {
            Direction::TopLeft
            | Direction::BottomRight
            | Direction::TopRight
            | Direction::BottomLeft => true,
            _ => false,
        })
        .cloned()
        .collect()
}

fn part2(puzzle: &Vec<Vec<char>>) {
    let mut total: i32 = 0;

    for (y_index, y) in puzzle.into_iter().enumerate() {
        let mut pos: Coordinate = Coordinate { x: 0, y: y_index };
        for (x_index, x) in y.iter().enumerate() {
            if *x != 'A' {
                continue;
            }
            pos.x = x_index;

            let m_around: Vec<Direction> = keep_corners(&look_around(&'M', puzzle, &pos));
            let s_around: Vec<Direction> = keep_corners(&look_around(&'S', puzzle, &pos));

            if m_around.len() != 2
                || s_around.len() != 2
                || !is_corner_next_to(&m_around[0], &m_around[1])
            {
                continue;
            }

            total += 1;
        }
    }
    println!("Part 2 result: {total}");
}

fn main() {
    let puzzle: Vec<Vec<char>> = match parse_puzzle() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error parsing puzzle: {}", e);
            return;
        }
    };

    part1(&puzzle);
    part2(&puzzle);
}
