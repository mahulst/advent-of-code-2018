extern crate linked_list;

use linked_list::{LinkedList, Cursor};
use std::collections::HashMap;

pub fn add_marble(cursor: &mut Cursor<u32>, marble: u32) -> u32 {
    if marble % 23 == 0 {
        go_to_position(cursor, 7, Direction::Backward);
        marble + cursor.remove().unwrap()
    } else {
        go_to_position(cursor, 2, Direction::Forward);
        cursor.insert(marble);
        0
    }
}

pub fn play_game(last_marble: u32, amount_of_players: u32) -> u32 {
    let mut players: HashMap<u32, u32> = HashMap::new();
    let mut marble = 1;
    let mut done = false;

    let mut circle = LinkedList::new();
    circle.push_back(0);
    let mut cursor = circle.cursor();

    while !done {
        for x in 0..amount_of_players {
            let score = add_marble(&mut cursor, marble);

            let player_score = players.entry(x).or_insert(0);
            *player_score += score;

            marble += 1;

            if last_marble < marble {
                done = true;
                break;
            }
        }
    }

    players.into_iter().fold(0, |acc, (_, result)| {
        if acc > result {
            acc
        } else {
            result
        }
    })
}

enum Direction {
    Forward,
    Backward,
}

fn go_to_position(cursor: &mut Cursor<u32>, n: isize, direction: Direction) {
    for _ in 0..n {
        match direction {
            Direction::Forward => {
                if cursor.next().is_none() {
                    cursor.next();
                }
            }
            Direction::Backward => {
                if cursor.prev().is_none() {
                    cursor.prev();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::play_game;
    use crate::add_marble;
    use linked_list::LinkedList;

    #[test]
    fn it_should_add_a_marble_in_correct_place() {
        // Arrange
        let mut circle = LinkedList::new();
        circle.push_back(0);
        let mut cursor = circle.cursor();

        // Act

        for i in 1..23 {
            add_marble(&mut cursor, i);
        }

        // Assert
        let result: Vec<u32> = circle.into_iter().collect();
        assert_eq!(result, vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]);
    }

    #[test]
    fn should_return_the_correct_score() {
        // Arrange
        let mut circle = LinkedList::new();
        circle.push_back(0);
        let mut cursor = circle.cursor();
        for i in 1..23 {
            add_marble(&mut cursor, i);
        }

        // Act
        let score = add_marble(&mut cursor, 23);
        let expected_result: Vec<u32> = vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15];
        let marbles: Vec<u32> = circle.into_iter().collect();


        // Assert
        assert_eq!(marbles, expected_result);
        assert_eq!(score, 9 + 23);
    }


    #[test]
    fn should_calculate_the_high_score() {
        // Act
        let result = play_game(1618, 10);
        let result2 = play_game(7999, 13);
        let result3 = play_game(1104, 17);
        let result4 = play_game(6111, 21);
        let result5 = play_game(5807, 30);

        // Assert
        assert_eq!(result, 8317);
        assert_eq!(result2, 146373);
        assert_eq!(result3, 2764);
        assert_eq!(result4, 54718);
        assert_eq!(result5, 37305);
    }
}
