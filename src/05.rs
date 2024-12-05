use itertools::Itertools;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::{collections::HashMap, env, fs};

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

#[derive(Debug)]
struct Rules {
    rules: HashMap<usize, Vec<usize>>,
}

impl Rules {
    fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    fn add_num_rule(&mut self, num: usize, must_precede: usize) {
        if self.rules.contains_key(&num) {
            self.rules.get_mut(&num).unwrap().push(must_precede);
        } else {
            self.rules.insert(num, Vec::from([must_precede]));
        }
    }

    fn num_must_precede_num(&self, num: usize, must_precede: usize) -> bool {
        self.rules
            .get(&num)
            .unwrap_or(&vec![])
            .contains(&must_precede)
    }
}

fn parse_input(input: &str) -> (Rules, Vec<Vec<usize>>) {
    let (rule_lines, update_lines) = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split("\n")
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
        })
        .collect_tuple()
        .unwrap();

    let mut rules = Rules::new();
    for line in rule_lines {
        let (num, must_precede) = line
            .split("|")
            .map(|n| n.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        rules.add_num_rule(num, must_precede);
    }

    let update_nums = update_lines
        .iter()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, update_nums)
}

fn part_1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    let correct_updates = updates
        .iter()
        .filter(|update| {
            for (i, num) in update.iter().enumerate() {
                if rules.rules.contains_key(num) {
                    let must_follow = rules.rules.get(num).unwrap();

                    for preceding_num in &update[0..=i] {
                        if must_follow.contains(preceding_num) {
                            return false;
                        }
                    }
                }
            }
            return true;
        })
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum::<usize>();

    correct_updates
}

fn part_2(input: &str) -> usize {
    let (rules, mut updates) = parse_input(input);

    updates
        .iter_mut()
        .filter(|update| {
            !update
                .is_sorted_by(|num, following_num| rules.num_must_precede_num(*num, *following_num))
        })
        .map(|update| {
            update.sort_by(|num, following_num| {
                if rules.num_must_precede_num(*num, *following_num) {
                    Less
                } else {
                    Greater
                }
            });

            update
        })
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum()
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
        assert_eq!(part_2(SAMPLE_DATA), 123);
    }
}
