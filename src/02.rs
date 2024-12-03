use itertools::Itertools;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/02.txt");
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
    let reports: Vec<Vec<usize>> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            return line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
        })
        .collect();

    let num_safe_reports: usize = reports
        .iter()
        .filter(|rep| {
            is_safe_report(rep)
        })
        .count();

    return num_safe_reports;
}

fn is_safe_report(report: &Vec<usize>) -> bool {
    return report.iter().tuple_windows().all(|(a, b, c)| {
        if a == b || b == c {
            return false;
        }
        if a > b && b < c || a < b && b > c {
            return false;
        }

        if !(a.abs_diff(*b) >= 1 && a.abs_diff(*b) <= 3) {
            return false;
        }

        if !(b.abs_diff(*c) >= 1 && b.abs_diff(*c) <= 3) {
            return false;
        }

        return true;
    });
}

fn part_2(input: &str) -> usize {
    let reports: Vec<Vec<usize>> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            return line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
        })
        .collect();

    let num_safe_reports: usize = reports
        .iter()
        .filter(|rep| {
            if is_safe_report(rep) {
                return true;
            }

            // Try all combinations when deleting ONE level
            for i in 0..rep.len() {
                let mut rep_copy = rep.to_vec();
                rep_copy.remove(i);
                if is_safe_report(&rep_copy) {
                    return true;
                }
            }
            return false;
        })
        .count();

    return num_safe_reports;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 4);
    }
}
