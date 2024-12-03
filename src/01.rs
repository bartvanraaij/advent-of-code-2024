use itertools::{Either, Itertools};
use rayon::prelude::*;
use std::{env, fs};

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/01.txt");
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

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .flat_map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec()
        })
        .enumerate()
        .partition_map(|(i, num)| {
            if i % 2 == 0 {
                Either::Left(num)
            } else {
                Either::Right(num)
            }
        })
}

fn part_1(input: &str) -> usize {
    let (l, r) = parse_input(input);

    let total_distance = (
        l.into_iter().sorted().collect_vec(),
        r.into_iter().sorted().collect_vec(),
    )
        .into_par_iter()
        .map(|(x, y)| x.abs_diff(y))
        .sum();

    return total_distance;
}

fn part_2(input: &str) -> usize {
    let (l, r) = parse_input(input);

    let similarity_scores: usize = l
        .into_iter()
        .map(|num| {
            let count = (*r).into_iter().filter(|x| *x == &num).count();
            return num * count;
        })
        .sum();

    return similarity_scores;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 31);
    }
}
