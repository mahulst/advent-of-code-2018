pub fn create_new_recipes(recipes: &Vec<u32>, active1: usize, active2: usize) -> (Option<u32>, Option<u32>) {
    let recipe_score1 = recipes.get(active1).expect("active 1 has invalid index");
    let recipe_score2 = recipes.get(active2).expect("active 2 has invalid index");

    let first_digit: u32 = (recipe_score1 + recipe_score2) % 10;
    let second_digit: u32 = ((recipe_score1 + recipe_score2) as f32 / 10.0).floor() as u32;

    if second_digit > 0 {
        (Some(second_digit), Some(first_digit))
    } else {
        (None, Some(first_digit))
    }
}

pub fn find_next_index(recipes: &Vec<u32>, active1: usize, active2: usize) -> (usize, usize) {
    let length = recipes.len();
    let recipe_score1 = recipes.get(active1).expect("active 1 has invalid index");
    let recipe_score2 = recipes.get(active2).expect("active 2 has invalid index");

    let index1 = (*recipe_score1) as usize + active1 + 1;
    let index2 = (*recipe_score2) as usize + active2 + 1;

    (index1 % length, index2 % length)
}

pub fn find_next_ten(recipes: &mut Vec<u32>, index: usize) -> &[u32] {
    let mut active1 = 0;
    let mut active2 = 1;
    while recipes.len() < index + 11 {
        let result = create_new_recipes(&recipes, active1, active2);

        if let Some(r) = result.0 {
            recipes.push(r);
        }
        if let Some(r) = result.1 {
            recipes.push(r);
        }

        let new_indices = find_next_index(&recipes, active1, active2);

        active1 = new_indices.0;
        active2 = new_indices.1;
    }

    &recipes[index..index + 10]
}

pub fn find_first_occurence(recipes: &mut Vec<u32>, digits: &[u32], window_size: usize, tries: usize) -> Option<usize> {


    let mut active1 = 0;
    let mut active2 = 1;
    let mut found = None;
    let mut current_try = 0;
    while current_try < tries {

        let result = create_new_recipes(&recipes, active1, active2);

        if let Some(r) = result.0 {
            recipes.push(r);
        }
        if let Some(r) = result.1 {
            recipes.push(r);
        }

        let new_indices = find_next_index(&recipes, active1, active2);

        active1 = new_indices.0;
        active2 = new_indices.1;


        current_try += 1;

    }

    let mut i = 0;
    if let Some(r) = recipes.windows(window_size).find(|slice| {
        i += 1;
        *slice == digits
    }) {
        found = Some(i - 1);
    }
    found

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_add_new_recipes() {
        // Arrange
        let mut recipes = vec![3, 7];

        // Act
        let result = crate::create_new_recipes(&recipes, 0, 1);

        // Assert
        assert_eq!(result, (Some(1), Some(0)));
    }

    #[test]
    fn it_should_only_add_one_new_recipe_for_score_lower_than_ten() {
        // Arrange
        let mut recipes = vec![3, 7, 0, 1, 0];

        // Act
        let result = crate::create_new_recipes(&recipes, 3, 4);

        // Assert
        assert_eq!(result, (None, Some(1)));
    }

    #[test]
    fn it_should_find_next_index() {
        // Arrange
        let recipes = vec![3, 7, 1, 0, 1, 0, 1];
        let active_1 = 4;
        let active_2 = 3;


        // Act
        let result = crate::find_next_index(&recipes, active_1, active_2);

        // Assert
        assert_eq!(result, (6, 4));
    }

    #[test]
    fn it_should_find_next_index_looping_around() {
        // Arrange
        let recipes = vec![3, 7, 1, 0];
        let active_1 = 0;
        let active_2 = 1;


        // Act
        let result = crate::find_next_index(&recipes, active_1, active_2);

        // Assert
        assert_eq!(result, (0, 1));
    }

    #[test]
    fn it_should_calculate_the_next_ten_recipes() {
        // Arrange
        let mut recipes = vec![3, 7];

        // Act
        let result = crate::find_next_ten(&mut recipes, 9);

        // Assert
        assert_eq!(result, &[5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
    }

    #[test]
    fn it_should_solve_part_1() {
        // Arrange
        let mut recipes = vec![3, 7];

        // Act
        let result = crate::find_next_ten(&mut recipes, 513401);

        // Assert
        assert_eq!(result, &[5, 3, 7, 1, 3, 9, 3, 1, 1, 3]);
    }

    #[test]
    fn it_should_find_the_first_occurence() {
        // Arrange
        let mut recipes = vec![3, 7];

        // Act
        let result = crate::find_first_occurence(&mut recipes, &[5,1,5,8,9], 5, 100);

        // Assert
        assert_eq!(result, Some(9));
    }

    #[test]
    fn it_should_find_the_first_occurence2() {
        // Arrange
        let mut recipes = vec![3, 7];

        // Act
        let result = crate::find_first_occurence(&mut recipes, &[5,9,4,1,4], 5, 3000);

        // Assert
        assert_eq!(result, Some(2018));
    }
}
