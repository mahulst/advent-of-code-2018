use rayon::prelude::*;

pub fn react(input: String) -> usize {
    let mut new_polymer = Vec::new();
    for c in input.chars() {
        match new_polymer.last() {
            None => new_polymer.push(c),
            Some(l) => {
                if is_opposite(c, *l) {
                    new_polymer.pop();
                } else {
                    new_polymer.push(c);
                }
            }
        }
    }

    new_polymer.len()
}

pub fn is_opposite(char1: char, char2: char) -> bool {
    let is_same_char = char1.eq_ignore_ascii_case(&char2);
    let but_not_the_same = char1 != char2;
    is_same_char && but_not_the_same
}

pub fn remove_char_from_string(string: &str, char1: &char, char2: &char) -> String {
    let result = string.replace(*char1, "");
    result.replace(*char2, "")
}

const LOWERCASE_CHARS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const UPPERCASE_CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub fn find_most_blocking_unit(string: &str) -> usize {
    LOWERCASE_CHARS
        .par_iter()
        .zip(UPPERCASE_CHARS.par_iter())
        .map(|(l, u)| {
            let string = remove_char_from_string(string, l, u);
            react(string)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::remove_char_from_string;
    use crate::react;

    #[test]
    fn it_should_remove_chars_from_string() {

        // Arrange
        let input = "BaAbBAaBBA";

        // Act
        let result = remove_char_from_string(input, &'a', &'A');

        // Assert
        assert_eq!(result, "BbBBB");
    }

    #[test]
    fn it_should_react_opposite_units() {

        // Arrange
        let input = String::from("aA");
        let input1 = String::from("Bb");

        // Act
        let result = react(input);
        let result1 = react(input1);

        // Assert
        assert_eq!(result, 0);
        assert_eq!(result1, 0);
    }

    #[test]
    fn it_should_not_react_different_units() {

        // Arrange
        let input = String::from("abAB");

        // Act
        let result = react(input);

        // Assert
        assert_eq!(result, 4);
    }

    #[test]
    fn it_should_not_react_not_opposite_units() {

        // Arrange
        let input = String::from("aabAAB");

        // Act
        let result = react(input);

        // Assert
        assert_eq!(result, 6);
    }

    #[test]
    fn it_should_react_opposite_units_recursively() {

        // Arrange
        let input = String::from("abBA");

        // Act
        let result = react(input);

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn it_should_result_in_a_stable_polymer() {

        // Arrange
        let input = String::from("dabAcCaCBAcCcaDA");

        // Act
        let result = react(input);

        // Assert
        assert_eq!(result, 10);
    }
}
