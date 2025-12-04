advent_of_code::solution!(4);

fn get_adjacent(x: usize, y: usize, grid: &Vec<Vec<char>>) -> usize {
    let mut adjacent = 0;

    let delta: [isize; 3] = [-1, 0, 1];
    for dx in delta {
        for dy in delta {
            let x_coord = dx + x as isize;
            let y_coord = dy + y as isize;
            if x_coord < 0
                || y_coord < 0
                || x_coord >= grid[0].len() as isize
                || y_coord >= grid.len() as isize
            {
                continue;
            }

            if grid[y_coord as usize][x_coord as usize] != '.' {
                adjacent += 1;
            }
        }
    }
    adjacent - 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut paper_rolls = Vec::new();
    for line in input.lines() {
        let mut l = Vec::new();
        for c in line.chars() {
            l.push(c);
        }
        paper_rolls.push(l);
    }

    let mut accessible_rolls = 0;
    for y in 0..paper_rolls.len() {
        for x in 0..paper_rolls[0].len() {
            if paper_rolls[y][x] == '@' {
                if get_adjacent(x, y, &paper_rolls) < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }
    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut paper_rolls = Vec::new();
    for line in input.lines() {
        let mut l = Vec::new();
        for c in line.chars() {
            l.push(c);
        }
        paper_rolls.push(l);
    }

    let mut removed_rolls = 0;
    loop {
        let mut progress = false;

        for y in 0..paper_rolls.len() {
            for x in 0..paper_rolls[0].len() {
                if paper_rolls[y][x] == '@' {
                    if get_adjacent(x, y, &paper_rolls) < 4 {
                        removed_rolls += 1;
                        paper_rolls[y][x] = 'x';
                        progress = true;
                    }
                }
            }
        }

        for y in 0..paper_rolls.len() {
            for x in 0..paper_rolls[0].len() {
                if paper_rolls[y][x] == 'x' {
                    paper_rolls[y][x] = '.';
                }
            }
        }

        if !progress {
            break;
        }
    }
    Some(removed_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
