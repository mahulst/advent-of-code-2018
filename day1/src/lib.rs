use std::str::Lines;
use std::collections::HashMap;

pub fn calibrate_frequency(lines: Lines) -> i32 {
    let mut freq = 0;

    for line in lines {
        let change = line.to_string().trim().parse::<i32>().unwrap();
        freq += change;
    }

    freq
}

pub fn find_first_duplicate(lines: Lines) -> i32 {
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    let iter = lines.clone().cycle();

    let mut freq = 0;
    for line in iter {
        let change = line.to_string().trim().parse::<i32>().unwrap();
        freq += change;

        if frequencies.contains_key(&freq) {
            return freq;
        } else {
            frequencies.insert(freq, 1);
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use calibrate_frequency;
    use find_first_duplicate;

    #[test]
    fn it_should_add_changes_to_frequency() {

        // Arrange
        let input = "+1
        +1
        +1";

        // Act
        let result = calibrate_frequency(input.lines());

        // Assert
        assert_eq!(result, 3);
    }

    #[test]
    fn it_should_subtract_negative_changes() {

        // Arrange
        let input = "+1
        +1
        -2";

        // Act
        let result = calibrate_frequency(input.lines());

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn it_should_return_the_first_duplicate_frequency() {

        // Arrange
        let input = "+1
        +2
        +3
        -6
        +1
        +2
        +3";

        // Act
        let result = find_first_duplicate(input.lines());

        // Assert
        assert_eq!(result, 1);
    }

    #[test]
    fn it_should_cycle_over_the_input() {

        // Arrange
        let input = "+1
        +1
        +1
        +1
        -5";

        // Act
        let result = find_first_duplicate(input.lines());

        // Assert
        assert_eq!(result, 1);
    }
}
