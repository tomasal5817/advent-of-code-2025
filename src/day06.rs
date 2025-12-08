const INPUT: &str = include_str!("../input/day06.txt");

fn part1(numbers: &[Vec<u64>], operation: &[char]) -> u64 {
    let mut total: u64 = 0;
    let len_operation = operation.len();
    let cols = numbers[0].len();
    let rows = numbers.len();

    assert_eq!(cols, len_operation);

    for j in 0..(cols) {
        let mut sum: u64 = numbers[0][j];

        for i in 1..(rows) {
            if operation[j] == '*' {
                sum *= numbers[i][j];
            }else { 
                sum += numbers[i][j];
            }
        }
     total += sum;
    }

 total
}

fn part2(input: &str) -> u64 { 
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut total: u64 = 0;
    let mut numbers: Vec<u64> = Vec::new();

    for j in (0..cols).rev() {
        let mut s = String::new();

        for i in 0..rows {
            let c = matrix[i][j];

            if c.is_ascii_digit() {
                s.push(c);
            }

            if i == rows - 1 {
                if ! s.is_empty() {
                    let num: u64 = s.parse().expect("could not parse number");
                    numbers.push(num);
                }
                
                if c == '+' {
                    total += numbers.iter().sum::<u64>();
                    numbers.clear();
                }else if c == '*'{
                    total += numbers.iter().product::<u64>();
                    numbers.clear();
                } 
            }
        }
    }

    total
} 


fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut lines = input.lines();

    let last_line = lines
        .next_back()
        .expect("input must have at least one line");
    
    let operation: Vec<char> = last_line
        .split_whitespace()     
        .map(|s| s.chars().next().unwrap())
        .collect();

    let numbers: Vec<Vec<u64>> = lines
        .map(|line| {
            line
                .split_whitespace()
                .map(|tok| tok.parse::<u64>().expect("Expect natural number"))
                .collect()
        })
        .collect();
   
    (numbers, operation)
}

fn main() {
    let (numbers, operation) = parse_input(&INPUT);
    println!("Part1 = {}", part1(&numbers, &operation));
    println!("Part2 = {}", part2(&INPUT));
}

