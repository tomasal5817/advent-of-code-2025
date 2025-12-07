const INPUT: &str = include_str!("../input/day04.txt");

fn part1(matrix: &[Vec<char>], threshold: usize) -> u32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut sum_paper: u32 = 0;
    let neighbors: [isize; 3] = [-1, 0, 1];
    
    for i in 0..rows{
        for j in 0..cols { 
            if matrix[i][j] == '.' {
                continue;
            }
            let mut count_neighbors: usize = 0; 

            for dx in neighbors {
                for dy in neighbors {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let row_idx = i as isize + dx;
                    let col_idx = j as isize + dy;

                    if 0 <= row_idx && row_idx < rows as isize && 0 <= col_idx && col_idx < cols as isize {
                        let r = row_idx as usize;
                        let c = col_idx as usize;

                        if matrix[r][c] == '@' {
                            count_neighbors += 1;
                        }
                    }
                }
            }
            if count_neighbors < threshold {
                sum_paper += 1;
            }

        }
    }

    sum_paper
}

fn part2(matrix: &[Vec<char>], threshold: usize) -> u32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut sum_paper: u32 = 0;
    let neighbors: [isize; 3] = [-1, 0, 1];
    let mut old_matrix = matrix.to_vec();
    let mut new_matrix = old_matrix.clone();


    loop {
        let mut paper_removed: bool = false;

        for i in 0..rows{
            for j in 0..cols { 
                if old_matrix[i][j] == '.' {
                    new_matrix[i][j] = '.';
                    continue;
                }
                let mut count_neighbors: usize = 0; 

                for dx in neighbors {
                    for dy in neighbors {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let row_idx = i as isize + dx;
                        let col_idx = j as isize + dy;

                        if 0 <= row_idx && row_idx < rows as isize && 0 <= col_idx && col_idx < cols as isize {
                            let r = row_idx as usize;
                            let c = col_idx as usize;

                            if old_matrix[r][c] == '@' {
                                count_neighbors += 1;
                            }
                        }
                    }
                }

                if count_neighbors < threshold {
                    sum_paper += 1;
                    new_matrix[i][j] = '.';
                    paper_removed = true;
                }else {
                    new_matrix[i][j] = '@';
                }
            }
        }

        if !paper_removed {
            break;
        }
        std::mem::swap(&mut old_matrix, &mut new_matrix);
    }

    sum_paper
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines();
    lines.map(|line| {
        let chars = line
            .chars()
            .collect();
        chars
    })
    .collect()
}

fn main() {
    let matrix = parse_input(INPUT);
    let threshold: usize = 4;

    println!{"Number of papper rolls part1 = {}", part1(&matrix, threshold)}
    println!{"Number of papper rolls part2 = {}", part2(&matrix, threshold)}
}