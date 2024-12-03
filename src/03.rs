use regex::Regex;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/03.txt");
    let input_filepath: &str = args.get(1).unwrap_or(default_input_filename);
    fs::read_to_string(input_filepath).expect("input file should be readable")
}

fn main() {
    let input = read_input_file(env::args().collect());
    let result_part_1 = part_1(&input);
    println!("{:?}", result_part_1);

    let result_part_2 = part_2(&input);
    println!("{:?}", result_part_2);
}

fn part_1(input: &str) -> usize {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .flat_map(|line| {
            let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
            return mul_regex
                .captures_iter(line)
                .map(|cap| {
                    let a = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    return a * b;
                })
                .collect::<Vec<usize>>();
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").filter(|l| !l.is_empty()).collect();

    let mut enabled = true;
    let mut sum = 0;
    let instr_regex = Regex::new(r"(mul|don't|do)\(((\d{1,3}),(\d{1,3}))?\)").unwrap();

    for line in lines {
        for cap in instr_regex.captures_iter(line) {
            let keyword = cap.get(1).unwrap().as_str();

            if keyword == "do" {
                enabled = true;
            }

            if keyword == "don't" {
                enabled = false;
            }

            if enabled && keyword == "mul" {
                let a = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let b = cap.get(4).unwrap().as_str().parse::<usize>().unwrap();
                sum += a * b;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests_00 {
    use super::*;

    const SAMPLE_DATA: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 161);
    }

    const SAMPLE_DATA_2: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA_2), 48);
    }
}
