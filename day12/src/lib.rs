use core::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Pot {
    Plant,
    Empty,
}

#[derive(PartialEq)]
pub struct Row {
    pub pots: Vec<Pot>,
    pub first_index: i32,
}

type Pattern = [Pot; 5];

pub fn input_to_list_with_index(input: &str, first_index: i32) -> Row {
    let mut pots = Vec::new();
    input.chars().for_each(|c| {
        match c {
            '#' => pots.push(Pot::Plant),
            _ => pots.push(Pot::Empty)
        };
    });

    Row { pots, first_index }
}

pub fn input_to_list(input: &str) -> Row {
    input_to_list_with_index(input, 0)
}


pub fn tick_row(patterns: &Vec<Pattern>, row: &mut Row) -> Row {
    let mut new_row = Vec::new();
    let first = row.pots.iter().nth(0).unwrap();
    let mut first_index = row.first_index;
    if *first == Pot::Plant {
        row.pots.insert(0, Pot::Empty);
        first_index -= 1;
    }
    let last_index = row.pots.len();
    let second_last = row.pots.iter().nth(last_index - 2).unwrap();
    let last = row.pots.iter().nth(last_index - 1).unwrap();

    let mut extra = 0;
    if *second_last == Pot::Plant {
        extra = 1;
    }
    if *last == Pot::Plant {
        extra = 2;
    }

    for i in 0..row.pots.len() + extra {
        let pattern_row = get_pattern_on_row_index(&row, i);

        if patterns.contains(&pattern_row) {
            new_row.push(Pot::Plant);
        } else {
            new_row.push(Pot::Empty);
        }
    };

    Row { pots: new_row, first_index }
}

pub fn get_pattern_on_row_index(row: &Row, index: usize) -> Pattern {
    let mut pattern = [Pot::Empty; 5];

    let mut iter = row.pots.iter();
    for i in 0..index + 3 {
        let pot = match iter.next() {
            Some(p) => *p,
            None => {
                Pot::Empty
            }
        };
        if (i as i32) >= (index as i32) - 2 && i <= index + 2 {
            pattern[2 + i - index] = pot;
        }
    }

    pattern
}

pub fn parse_pattern(line: &str) -> Pattern {
    let mut pattern: [Pot; 5] = [Pot::Empty; 5];
    let mut i = 0;
    line.chars().take(5).for_each(|c| {
        pattern[i] = match c {
            '#' => Pot::Plant,
            _ => Pot::Empty
        };
        i += 1;
    });

    pattern
}

pub fn count_row(row: &Row) -> i32 {
    let mut i = row.first_index;
    let mut total = 0;

    row.pots.iter().for_each(|p| {
        match p {
            Pot::Empty => {}
            Pot::Plant => {
                total += i;
            }
        }
        i += 1;
    });

    total
}

#[cfg(test)]
mod tests {
    use crate::Pot::{Plant, Empty};
    use crate::input_to_list;
    use crate::parse_pattern;
    use crate::get_pattern_on_row_index;
    use crate::tick_row;
    use crate::Pattern;
    use crate::Row;
    use crate::input_to_list_with_index;
    use crate::count_row;

    #[test]
    fn it_should_parse_a_pattern() {
        // Arrange
        let input = "##.#. => #";
        let expected_result = [Plant, Plant, Empty, Plant, Empty];

        // Act
        let result = parse_pattern(input);
        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_should_convert_input_to_list_of_pots() {
        // Arrange
        let input = "#..#.#";
        let mut expected_list = Vec::new();
        expected_list.push(Plant);
        expected_list.push(Empty);
        expected_list.push(Empty);
        expected_list.push(Plant);
        expected_list.push(Empty);
        expected_list.push(Plant);

        // Act
        let result = input_to_list(input);

        // Assert
        assert_eq!(result, Row { pots: expected_list, first_index: 0 });
    }

    #[test]
    fn it_should_get_pattern_on_row_index() {
        // Arrange
        let row = input_to_list("#####");

        // Act
        let pattern_row = get_pattern_on_row_index(&row, 2);

        // Assert
        assert_eq!(pattern_row, [Plant, Plant, Plant, Plant, Plant]);
    }

    #[test]
    fn it_should_get_pattern_on_row_index2() {
        // Arrange
        let row = input_to_list(".....");

        // Act
        let pattern_row = get_pattern_on_row_index(&row, 2);

        // Assert
        assert_eq!(pattern_row, [Empty, Empty, Empty, Empty, Empty]);
    }

    #[test]
    fn it_should_get_pattern_on_row_index_at_the_end() {
        // Arrange
        let row = input_to_list("#####..#");

        // Act
        let pattern_row = get_pattern_on_row_index(&row, 6);

        // Assert
        assert_eq!(pattern_row, [Plant, Empty, Empty, Plant, Empty]);
    }

    #[test]
    fn it_should_get_pattern_on_row_index_at_the_beginning() {
        // Arrange
        let row = input_to_list("#.###..#");

        // Act
        let pattern_row = get_pattern_on_row_index(&row, 0);

        // Assert
        assert_eq!(pattern_row, [Empty, Empty, Plant, Empty, Plant]);
    }

    #[test]
    fn it_should_return_a_new_row_of_plants() {
        // Arrange
        let mut initial_state = input_to_list(".#..#.#..##......###...###.");
        let expected_new_row = input_to_list(".#...#....#.....#..#..#..#..");
        let patterns = get_patterns();

        // Act
        let new_row = tick_row(&patterns, &mut initial_state);


        // Assert
        assert_eq!(new_row, expected_new_row);
    }

    #[test]
    fn it_should_return_a_new_row_of_plants_and_update_first_index() {
        // Arrange
        let mut initial_state = input_to_list("#.#...#..#.#....#..#..#...#");
        let expected_new_row = input_to_list_with_index("..#.#..#...#.#...#..#..##..##.", -1);
        let patterns = get_patterns();

        // Act
        let new_row = tick_row(&patterns, &mut initial_state);


        // Assert
        assert_eq!(new_row, expected_new_row);
    }

    #[test]
    fn it_should_tick_over_multiple_generations() {
        // Arrange
        let mut initial_state = input_to_list(
            "#..#.#..##......###...###..........."
        );
        let expected_new_row = input_to_list_with_index(
            ".#....##....#####...#######....#.#..##.", -3,
        );
        let patterns = get_patterns();

        // Act
        let mut new_row: Row = tick_row(&patterns, &mut initial_state);
        for _ in 0..19 {
            new_row = tick_row(&patterns, &mut new_row);
        }

        // Assert
        assert_eq!(new_row, expected_new_row);
    }

    #[test]
    fn it_should_count_plants() {
        // Arrange
        let mut initial_state = input_to_list(
            "#..#.#..##......###...###..........."
        );
        let patterns = get_patterns();
        let mut new_row: Row = tick_row(&patterns, &mut initial_state);

        for _ in 0..19 {
            new_row = tick_row(&patterns, &mut new_row);
        }

        // Act
        let result = count_row(&new_row);

        // Assert
        assert_eq!(result, 325);
    }

    fn get_patterns() -> Vec<Pattern> {
        let input = "...##
..#..
.#...
.#.#.
.#.##
.##..
.####
#.#.#
#.###
##.#.
##.##
###..
###.#
####.
";
        input.lines().map(|l| parse_pattern(l)).collect()
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        self.pots.iter().for_each(|p| {
            match p {
                Pot::Empty => s.push('.'),
                Pot::Plant => s.push('#'),
            }
        });

        write!(f, "{} with first index: {}", s, self.first_index)
    }
}