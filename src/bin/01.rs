advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeros = 0;
    for turn in input.lines() {
        let mut chars = turn.chars();
        let direction = chars.next().unwrap();
        let mut steps = chars.collect::<String>().parse::<i32>().unwrap();

        if direction == 'L' {
            steps *= -1;
        }

        dial += steps;
        while dial < 0 {
            dial += 100;
        }
        while dial > 99 {
            dial -= 100;
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeros = 0;
    for turn in input.lines() {
        let mut chars = turn.chars();
        let direction = chars.next().unwrap();
        let mut steps = chars.collect::<String>().parse::<i32>().unwrap();

        // naive
        let increment = if direction == 'L' { -1 } else { 1 };

        while steps > 0 {
            dial += increment;
            steps -= 1;

            while dial < 0 {
                dial += 100;
            }
            while dial > 99 {
                dial -= 100;
            }

            if dial == 0 {
                zeros += 1;
            }
        }
    }

    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
