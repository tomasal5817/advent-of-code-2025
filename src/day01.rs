use std::fs::read_to_string;

fn part1(input: &str) -> i32 {
    let mut result = 0;
    let mut dial = 50;

    for line in input.lines() {

        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let tics: i32 = chars.collect::<String>().parse().unwrap();
        
        if direction == 'R' {
            dial = (dial + tics).rem_euclid(100);
        } else {
            dial = (dial - tics).rem_euclid(100);
        }

        if dial == 0 {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> i32 {
     
    let mut result = 0;
    let mut dial = 50;

    for line in input.lines() {
        
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let tics: i32 = chars.collect::<String>().parse().unwrap();

         if direction == 'R' {
            result += (dial + tics) / 100;
            dial = (dial + tics).rem_euclid(100);
        } else {
            if dial == 0 {
                result += tics / 100;
            }else if tics >= dial {
                result += 1 + (tics - dial) / 100;
            }
            dial = (dial - tics).rem_euclid(100);
        }
    }

    result
}

fn main() {
    let input = read_to_string("../input/day01.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}