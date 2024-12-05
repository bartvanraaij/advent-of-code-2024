use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/05.txt");
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
    //input.split("\n").filter(|l| !l.is_empty()).map(|line| {});

    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 0);
    }
}
