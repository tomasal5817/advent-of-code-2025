const INPUT: &str = include_str!("../input/day05.txt");

fn part1(intervals: &[(u64, u64)], ingredients: &[u64]) -> u64 {
    let mut fresh_ingredient: u64 = 0; 

    for &ingredient in ingredients {

        for &(a, b) in intervals {
            if ingredient >= a && ingredient <= b {
                fresh_ingredient += 1;
                break;
            }
        }
    }

    fresh_ingredient
}

fn part2(intervals: &[(u64, u64)]) -> u64 {
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|&(start, _)| start);
    let mut merged_intervals = Vec::new();
    
    let mut fresh_ingredient: u64 = 0; 
    let mut current = intervals[0];

    for &(start, end) in intervals.iter().skip(1) {
        if start <= current.1 {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged_intervals.push(current);
            current = (start, end);
        }
    }
    merged_intervals.push(current);

    for (start, end) in merged_intervals {
        if start < end {
            fresh_ingredient += end - start + 1;
        }else {
            fresh_ingredient += 1; // start == end
        }
    }
    
    fresh_ingredient 
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (range_ingredient, numbers_ingredient) = input.split_once("\n\n").unwrap(); 

    let intervals = range_ingredient.lines().map(|line| {
        let (a, b) = line.split_once('-').unwrap();
        (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
    }).collect();

    let numbers = numbers_ingredient.lines().map(|line| 
        line.parse::<u64>().unwrap()).collect();

    (intervals, numbers)
}

fn main() {
    let (intervals, ingredients) = parse_input(&INPUT);
    let fresh_ingredient = part1(&intervals, &ingredients);
    println!("Number of fresh ingredients part1 = {}", fresh_ingredient);

    // Part 2    
    let total_fresh_ingredient = part2(&intervals);
     println!("Number of fresh ingredients part2 = {}", total_fresh_ingredient);
}

