advent_of_code::solution!(6);

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => panic!("Invalid operation character"),
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            numbers: Vec::<u64>::new(),
            operation: Operation::Add,
        }
    }
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => {
                self.numbers.iter().sum()
            }
            Operation::Multiply => {
                self.numbers.iter().product()
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let number_of_problems: usize = input.lines().nth(0).unwrap().split_whitespace().count();
    let mut problems: Vec<Problem> = vec![Problem::default(); number_of_problems];

    for line in input.lines() {
        let input_type = line
            .split_whitespace()
            .take(1)
            .find(|&s| s.contains('+') || s.contains('*'));

        match input_type {
            None => {
                let numbers: Vec<usize> = line
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();

                assert!(numbers.len() == number_of_problems);

                for i in 0..numbers.len() {
                    problems[i].numbers.push(numbers[i] as u64);
                }
            }
            Some(_) => {
                let operations: Vec<char> = line
                    .split_whitespace()
                    .map(|n| n.chars().nth(0).unwrap())
                    .collect();

                assert!(operations.len() == number_of_problems);

                for i in 0..operations.len() {
                    problems[i].operation = Operation::from_char(operations[i]);
                }
            }
        }
    }

    //println!("{:?}", problems);

    let mut sum = 0;
    for p in problems {
        sum += p.solve();
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let number_of_problems: usize = input.lines().nth(0).unwrap().split_whitespace().count();
    let mut problems: Vec<Problem> = vec![Problem::default(); number_of_problems];
    let mut numbers: Vec<Vec<char>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    for line in input.lines() {
        let input_type = line
            .split_whitespace()
            .take(1)
            .find(|&s| s.contains('+') || s.contains('*'));

        match input_type {
            None => {
                let num: Vec<char> = line.chars().collect();

                numbers.push(num);
            }
            Some(_) => {
                operations = line.chars().collect();
            }
        }
    }

    let mut problem_index = 0;
    let mut number = String::from("");
    for o in 0..operations.len() {
        number.clear();

        if operations[o] != ' ' {
            problems[problem_index].operation = Operation::from_char(operations[o]);
            problem_index += 1;
        }

        for i in 0..numbers.len() {
            number.push(numbers[i][o]);
        }

        match number.trim().parse::<usize>() {
            Ok(n) => problems[problem_index - 1].numbers.push(n as u64),
            Err(_) => (),
        }
    }

    let mut sum = 0;
    for p in problems {
        sum += p.solve();
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
