const INPUT: &str = include_str!("../input/day02.txt");

fn part1(tuples: &Vec<(u64, u64)>) -> u64 {
     
    let mut sum: u64 = 0;

    for &(a, b) in tuples {
        for n in a..=b {
            let s = n.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let half = s.len() / 2;
            let s_first = &s[..half];
            let s_second = &s[half..];
            if s_first == s_second{
                sum += n;
            }
        }
    }

    sum
}

fn part2(tuples: &Vec<(u64, u64)>) -> u64 {
     
    let mut sum: u64 = 0;

    for &(a, b) in tuples {
        for n in a..=b {
            let s = n.to_string();
            let t = format!("{}{}", s, s);
            let trimmed = &t[1..t.len() - 1];

            if trimmed.contains(&s) {
                sum += n;
            }
        }
    }

    sum
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input.split(',').map(|pair| {
        let mut nums = pair.split('-');
        let a: u64 = nums.next().unwrap().trim().parse().unwrap();
        let b: u64 = nums.next().unwrap().trim().parse().unwrap();
        (a, b)
    }) 
    .collect()
}

fn main() {
    let tuples = parse_input(INPUT); 
    println!("Part 1: {}", part1(&tuples));
    println!("Part 2: {}", part2(&tuples));
}