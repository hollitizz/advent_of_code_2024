use std::fs::File;

fn parse_puzzle() -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let mut file = File::open("./puzzle.txt")?;
    let mut contents = String::new();

    use std::io::Read;
    file.read_to_string(&mut contents)?;

    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    for line in contents.split("\n") {
        let content: Vec<&str> = line.split("   ").collect();

        let a: i32 = content[0].trim().parse().unwrap();
        let b: i32 = content[1].trim().parse().unwrap();

        list_a.push(a);
        list_b.push(b);
    }

    Ok((list_a, list_b))
}

fn part1() {
    let mut res: i32 = 0;

    let (mut list_a, mut list_b) = match parse_puzzle() {
        Ok((a, b)) => (a, b),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    list_a.sort();
    list_b.sort();

    for (a, b) in list_a.iter().zip(list_b.iter()) {
        res += (a - b).abs()
    }

    println!("Part 1 result: {}", res);
}

fn part2() {
    let mut res: i32 = 0;

    let (list_a, list_b) = match parse_puzzle() {
        Ok((a, b)) => (a, b),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    for a in list_a {
        let occ: usize = list_b.iter().filter(|&b| a == *b).count();

        res += a * occ as i32;
    }

    println!("Part 2 result: {}", res);
}

fn main() {
    part1();
    part2();
}
