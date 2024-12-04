use itertools::Itertools;
use num::ToPrimitive;
use std::cmp;
use std::collections::HashMap;
use std::iter;
use std::iter::zip;
use std::option::Iter;
use std::{env, fs};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct XY(i32, i32);

#[derive(Debug, EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl XY {
    fn right(self) -> XY {
        XY(self.0 + 1, self.1)
    }
    fn left(self) -> XY {
        XY(self.0 - 1, self.1)
    }
    fn top(self) -> XY {
        XY(self.0, self.1 - 1)
    }
    fn bottom(self) -> XY {
        XY(self.0, self.1 + 1)
    }
    fn top_left(self) -> XY {
        XY(self.0 - 1, self.1 - 1)
    }
    fn top_right(self) -> XY {
        XY(self.0 + 1, self.1 - 1)
    }
    fn bottom_left(self) -> XY {
        XY(self.0 - 1, self.1 + 1)
    }
    fn bottom_right(self) -> XY {
        XY(self.0 + 1, self.1 + 1)
    }
    fn surround(self) -> Vec<XY> {
        let mut vec = Vec::from([self.right(), self.bottom_right(), self.bottom()]);
        if self.0 > 0 {
            vec.push(self.bottom_left());
            vec.push(self.left());
        }
        if self.0 > 0 && self.1 > 0 {
            vec.push(self.top_left());
        }
        if self.1 > 0 {
            vec.push(self.top());
            vec.push(self.top_right());
        }
        vec
    }
}

#[derive(Debug, Clone)]
struct Char {
    char: char,
    pos: XY,
}

struct Puzzle {
    chars: HashMap<XY, Char>,
}

impl Puzzle {
    fn char_at(&self, xy: XY) -> Option<&Char> {
        self.chars.get(&xy)
    }

    fn char_at_str(&self, xy: XY) -> String {
        let c = self.char_at(xy);
        match c {
            None => String::from(""),
            Some(i) => i.char.to_string(),
        }
    }

    fn get_word_at_location(&self, xy: XY, direction: Direction, length: Option<i32>) -> String {
        let l = length.unwrap_or(4 as i32);

        let x = xy.0;
        let y = xy.1;

        let xrange: Box<dyn ExactSizeIterator<Item = i32>> = match direction {
            Direction::N | Direction::S => Box::new(iter::repeat_n(x, l as usize)),
            Direction::SW | Direction::W | Direction::NW => Box::new((x-l+1 .. x+1).rev()),
            Direction::NE | Direction::E | Direction::SE => Box::new(x..(x + l+1)),
        };

        let yrange: Box<dyn ExactSizeIterator<Item = i32>> = match direction {
            Direction::W | Direction::E => Box::new(iter::repeat_n(y, l as usize)),
            Direction::NE | Direction::N | Direction::NW => Box::new(((y-l+1)..y+1).rev()),
            Direction::SE | Direction::S | Direction::SW => Box::new(y..(y + l)),
        };

        let mut vec: Vec<String> = Vec::new();
        for (xr, yr) in zip(xrange, yrange) {
            let c = self.char_at_str(XY(xr,yr));
            vec.push(c);
        }

        return String::from(vec.join(""));
    }

    fn get_xmas_at_location(&self, xy: XY, length: Option<i32>) -> String {
        let l = length.unwrap_or(4 as i32);

        let x = xy.0;
        let y = xy.1;

        let mut adjecent_positions: Vec<XY> = Vec::new();

        adjecent_positions.push(XY(x-1,y-1));
        adjecent_positions.push(XY(x-1,y+1));
        adjecent_positions.push(XY(x+1,y-1));
        adjecent_positions.push(XY(x+1,y+1));
        //
        //
        // let xrange: Box<dyn ExactSizeIterator<Item = i32>> = match direction {
        //     Direction::N | Direction::S => Box::new(iter::repeat_n(x, l as usize)),
        //     Direction::SW | Direction::W | Direction::NW => Box::new((x-l+1 .. x+1).rev()),
        //     Direction::NE | Direction::E | Direction::SE => Box::new(x..(x + l+1)),
        // };
        //
        // let yrange: Box<dyn ExactSizeIterator<Item = i32>> = match direction {
        //     Direction::W | Direction::E => Box::new(iter::repeat_n(y, l as usize)),
        //     Direction::NE | Direction::N | Direction::NW => Box::new(((y-l+1)..y+1).rev()),
        //     Direction::SE | Direction::S | Direction::SW => Box::new(y..(y + l)),
        // };

        let mut vec: Vec<String> = Vec::new();
        for (xy) in adjecent_positions.iter() {
            let c = self.char_at_str(*xy);
            vec.push(c);
        }

        return String::from(vec.join(""));
    }

}

fn read_input_file(args: Vec<String>) -> String {
    let default_input_filename = &String::from("input/04.txt");
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

fn parse_input(input: &str) -> Puzzle {
    let chars = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            return line.chars().enumerate().map(move |(x, char)| {
                let xy = XY(x.try_into().unwrap(), y.try_into().unwrap());
                (xy, Char { char, pos: xy })
            });
        })
        .collect::<HashMap<_, _>>();

    return Puzzle { chars };
}

fn part_1(input: &str) -> usize {

    let puzzle = parse_input(input);

    puzzle
        .chars
        .values()
        .map(|char| {
            let mut count = 0;
            if char.char.to_string() == "X" {
                for direction in Direction::iter() {
                    dbg!(&direction, char.pos);
                    let word = puzzle.get_word_at_location(char.pos, direction, None);


                    dbg!(&word);
                    dbg!("=======");

                    if word == "XMAS" {
                        count += 1;
                    }
                }
        }
            count
        })
        .sum()
}

fn part_2(input: &str) -> usize {

    let puzzle = parse_input(input);

    puzzle
        .chars
        .values()
        .map(|char| {
            let mut count = 0;
            if char.char.to_string() == "X" {
                let word = puzzle.get_xmas_at_location(char.pos, None);
                if word == "SSMM" || word == "SMSM" || word == "MSMS" || word == "MMSS" {
                    count += 1;
                }
            }
            count
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(SAMPLE_DATA), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(SAMPLE_DATA), 9);
    }
}
