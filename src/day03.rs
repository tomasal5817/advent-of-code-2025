const INPUT: &str = include_str!("../input/day03.txt");

fn part1(matrix: &Vec<Vec<u64>>) -> u64 {

    let mut sum = 0;
    
    for row in matrix {
        let mut jolt1 = 0;
        let mut jolt2 = 0; 
        let range = row.len() - 1;
        let mut flag: bool = false;

        for (i, val) in row.iter().enumerate() {
            if jolt1 < *val && i <  range {
                jolt1 = *val;
                flag = true;
            } else if jolt2 < *val {
                jolt2 = *val;
            }
            if flag {
                jolt2 = row[i + 1];
                flag = false;
            }
        }
        sum += 10 * jolt1 + jolt2;
    }

    sum
}

fn part2(matrix: &Vec<Vec<u64>>, battery: usize) -> u64 {
    let mut sum: u64 = 0;

    for row in matrix {
        let n: usize = row.len();
        assert! (battery <= n, "Row to short: len = {}, battery = {} ", n, battery);

        let mut start_idx: usize = 0;
        let mut stop_idx: usize = n - battery;
        let mut total_jolt: u64 = 0;

            for i in (0..battery).rev() {
                let mut jolt = row[start_idx];
                let mut max_idx = start_idx + 1;

                for idx in (start_idx + 1)..=stop_idx {
                    if jolt < row[idx] {
                        jolt = row[idx];
                        max_idx = idx + 1;
                    }
                }
        
                start_idx = max_idx;
                total_jolt += jolt * 10u64.pow(i as u32);
                stop_idx += 1;
                
            }

        sum += total_jolt;
    }

    sum 
}

fn parse_input(input: &str) -> Vec<Vec<u64>>{
    let lines = input.lines();
    lines.map(|line| {
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        digits
    })
    .collect()
}

fn main() {
    let battery: usize = 12;
    let matrix = parse_input(INPUT);
    println!("Solution part1 = {}", part1(&matrix));
    println!("Solution part2 = {}", part2(&matrix, battery));
}
