use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn get_checksum(string: &str) -> i32 {
    let mut doubles = 0;
    let mut triples = 0;

    for line in string.lines() {
        let double_chars = has_char_counts(line, 2);
        let triple_chars = has_char_counts(line, 3);

        if double_chars {
            doubles += 1;
        }

        if triple_chars {
            triples += 1;
        }
    }

    doubles * triples
}

pub fn get_similar_chars(id1: &str, id2: &str) -> Vec<char> {
    id1
        .chars()
        .zip(id2.chars())
        .filter_map(|(a, b)| {
            match a == b {
                true => Some(a),
                false => None
            }
        })
        .collect()
}

pub fn count_not_similar_chars(id1: &str, id2: &str) -> usize {
    let similar_chars_len = get_similar_chars(id1, id2)
        .iter()
        .count();

    id1.len() - similar_chars_len
}

pub fn get_similar_ids(input: &str) -> HashSet<&str> {
    let result: Vec<Vec<&str>> = input.lines()
        .map(|id| id.trim())
        .map(|id1: &str| {
            input.lines()
                .map(|id| id.trim())
                .filter_map(|id2: &str| {
                    match count_not_similar_chars(id1, id2) {
                        1 => Some(id1),
                        _ => None
                    }
                })
                .collect()
        })
        .collect();

    let result2: Vec<&str> = result.into_iter().flatten().collect();

    HashSet::from_iter(result2)
}

pub fn has_char_counts(id: &str, count: i32) -> bool {
    let mut chars: HashMap<char, i32> = HashMap::new();

    for c in id.chars() {
        let amount = chars.entry(c).or_insert(0);

        *amount += 1;
    }

    chars.retain(|_, value| value == &count);

    chars.len() > 0
}


#[cfg(test)]
mod tests {
    use get_checksum;
    use count_not_similar_chars;
    use get_similar_ids;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn it_should_calculate_the_product_of_doubles_and_triples() {

        // Arrange
        let input = "abcdef
        bababc
        abbcde
        abcccd
        aabcdd
        abcdee
        ababab";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 12);
    }

    #[test]
    fn it_should_not_count_ids_with_no_doubles_or_triples() {

        // Arrange
        let input = "abcdef";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn it_should_count_both_triples_and_doubles() {

        // Arrange
        let input = "bababc";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_count_doubles() {

        // Arrange
        let input = "abbcde
        aaa";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_count_triples() {

        // Arrange
        let input = "abcccd
        aa";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_count_doubles_only_once() {

        // Arrange
        let input = "aabcdd
        aaa";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_count_triples_only_once() {

        // Arrange
        let input = "ababab
        aa";

        // Act
        let result = get_checksum(input);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_count_similar_chars_in_string() {

        // Arrange
        let input1 = "abcd";
        let input2 = "abce";

        // Act
        let result = count_not_similar_chars(input1, input2);

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_return_a_list_of_similar_ids() {

        // Arrange
        let input = "abcde
        fghij
        klmno
        pqrst
        fguij
        axcye
        wvxyz";
        let expected_result: HashSet<&str> = HashSet::from_iter(vec!["fguij", "fghij"]);

        // Act
        let result = get_similar_ids(input);

        // Assert
        assert_eq!(result, expected_result);
    }
}
