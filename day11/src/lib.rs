use core::cmp;

const GRID_WITDH: u32 = 300;

fn get_third_digit(power: u32) -> i32 {
    ((power as f32) / 100.0) as i32 % 10
}

pub fn get_power_level(input: u32, x: u32, y: u32) -> i32 {
    let rackid = x+ 10;
    let power_level = y*rackid;
    let power_level = power_level + input;
    let power_level = power_level * rackid;
    let power_level = get_third_digit(power_level);
    power_level - 5
}

pub fn get_power_level_for_square_size(grid: &Vec<i32>, x_coord: u32, y_coord: u32, square_size: u32) -> i32 {
    let mut power_level = 0;
    for y in y_coord..y_coord + square_size {
        for x in x_coord..x_coord + square_size {
            let index = (x - 1 + (y - 1) * 299) as usize;
            let local_power_level = grid.get(index).unwrap();
            power_level += local_power_level;
        }
    }
    power_level
}

pub fn build_grid_sizes(input: u32) -> Vec<i32> {
    let mut grid = vec![];
    for y in 1..GRID_WITDH {
        for x in 1..GRID_WITDH {
            let power_level = get_power_level(input, x, y);
            grid.push(power_level);
        }
    }

    grid
}

pub fn get_largest_cell_of_any_size(input: u32) -> (u32, u32, i32) {
    let mut largest = (999, 999, 999, 0);
    let grid = build_grid_sizes(input);
    for y in 1..GRID_WITDH {
        for x in 1..GRID_WITDH {
            let max_size = GRID_WITDH - cmp::max(x, y);

            for size in 1..max_size {
                let power_level = get_power_level_for_square_size(&grid, x, y, size);
                if largest.3 < power_level {
                    largest = (x, y, size as i32, power_level);
                }
            }
        }
    }

    (largest.0, largest.1, largest.2)
}

pub fn get_largest_cell(input: u32) -> (u32, u32) {
    let mut largest = (999, 999, 0);
    let grid = build_grid_sizes(input);
    for x in 1..298 {
        for y in 1..298 {
            let power_level = get_power_level_for_square_size(&grid, x, y, 3);

            if largest.2 < power_level {
                largest = (x, y, power_level);
            }
        }
    }

    (largest.0, largest.1)
}

#[cfg(test)]
mod tests {
    use crate::get_power_level;
    use crate::get_largest_cell;
    use crate::build_grid_sizes;
    use crate::get_power_level_for_square_size;

    #[test]
    fn it_should_get_power_level() {
        
        let input = 8;

        // Act
        let result = get_power_level(input, 3, 5);

        // Assert
        assert_eq!(result, 4);
    }

    #[test]
    fn it_should_build_a_grid_index_correctly() {
        // Arrange
        let input = 18;
        let grid = build_grid_sizes(input);

        // Act

        let power: &i32 = grid.get(32 + 44 * 299).unwrap();
        // Assert
        assert_eq!(*power, 4);
    }

    #[test]
    fn it_should_get_3x3_power_level() {
        // Arrange
        let input = 18;
        let grid = build_grid_sizes(input);

        // Act
        let result = get_power_level_for_square_size(&grid, 33, 45, 3);

        // Assert
        assert_eq!(result, 29);
    }

    #[test]
    fn it_should_find_largest_coord() {
        // Arrange
        let input = 18;

        // Act
        let result = get_largest_cell(input);

        // Assert
        assert_eq!(result, (33, 45));
    }

    #[test]
    fn it_should_find_largest_coord2() {
        // Arrange
        let input = 42;

        // Act
        let result = get_largest_cell(input);

        // Assert
        assert_eq!(result, (21, 61));
    }
}
