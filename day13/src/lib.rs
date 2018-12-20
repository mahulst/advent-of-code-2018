use core::cmp;
use std::cmp::Ordering;
use std::thread;
use core::time;

pub fn get_width(input: &str) -> usize {
    input.lines().fold(0, |length, line| {
        cmp::max(length, line.len())
    })
}

pub fn find_char_in_string(input: &str, x: usize, y: usize) -> char {
    let line = input.lines().nth(y).unwrap();
    let c = line.chars().nth(x).unwrap();

    c
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
pub struct Cart {
    pub direction: Direction,
    pub turns: Turn,
    pub position: Point,
    pub crashed: bool,
}


impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.x.cmp(&other.x)
            }
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl Eq for Point {}

pub fn find_carts(input: &str) -> Vec<Cart> {
    let mut y = 0;
    let mut carts = Vec::new();

    input.lines().for_each(|l| {
        let mut x = 0;
        l.chars().for_each(|c| {
            let direction = match c {
                '>' => Some(Direction::East),

                '<' => Some(Direction::West),

                '^' => Some(Direction::North),

                'v' => Some(Direction::South),

                _ => None
            };
            let position = Point { x, y };
            let turns = Turn::Left;

            match direction {
                Some(direction) => carts.push(Cart { position, direction, turns, crashed: false }),
                _ => {}
            };

            x += 1;
        });

        y += 1;
    });

    carts
}

pub fn get_empty_tracks(input: &str) -> String {
    let empty_track = input.replacen('>', "-", 999);
    let empty_track = empty_track.replacen('<', "-", 999);
    let empty_track = empty_track.replacen('^', "|", 999);
    let empty_track = empty_track.replacen('v', "|", 999);
    empty_track
}

fn find_collisions(carts: &Vec<Cart>) -> Option<(Point, usize, usize)> {
    let mut i = 0;
    let mut result = None;
    carts
        .iter()
        .for_each(|cart| {
            let next_position = get_next_position(cart);
            let mut j = 0;
            carts.iter().for_each(|cart2| {
                let cart2_position = if cart.position > cart2.position {
                    get_next_position(cart2)
                } else {
                    cart2.position
                };

                if !cart.crashed && !cart2.crashed && next_position == cart2_position && cart2 != cart && result == None {
                    result = Some((next_position, cmp::min(i, j), cmp::max(i, j)));
                }
                j += 1;
            });

            i += 1;
        });

    result
}

fn get_next_position(cart: &Cart) -> Point {
    match cart.direction {
        Direction::North => {
            Point { x: cart.position.x, y: cart.position.y - 1 }
        }
        Direction::South => {
            Point { x: cart.position.x, y: cart.position.y + 1 }
        }
        Direction::East => {
            Point { x: cart.position.x + 1, y: cart.position.y }
        }
        Direction::West => {
            Point { x: cart.position.x - 1, y: cart.position.y }
        }
    }
}

pub fn move_carts<'a, 'b>(empty_tracks: &'a str, carts: &'b mut Vec<Cart>) -> Option<Point> {
    carts.sort_by(|a, b| {
        a.position.partial_cmp(&b.position).unwrap()
    });


    if let Some(result) = find_collisions(&carts) {
        let (left, right) = carts.split_at_mut(result.2);
        let cart1: &mut Cart = left.get_mut(result.1).unwrap();
        let cart2: &mut Cart = right.get_mut(0).unwrap();

        cart1.crashed = true;
        cart2.crashed = true;

        return Some(result.0);
    }
    carts
        .iter_mut()
        .for_each(|cart| {
            update_cart(empty_tracks, cart);
        });

    None
}


pub fn update_cart(empty_tracks: &str, cart: &mut Cart) {
    let next_position = get_next_position(cart);

    let c = find_char_in_string(empty_tracks, next_position.x, next_position.y);
    match c {
        '\\' => {
            match cart.direction {
                Direction::North => {
                    cart.direction = Direction::West;
                }
                Direction::South => {
                    cart.direction = Direction::East;
                }
                Direction::East => {
                    cart.direction = Direction::South;
                }
                Direction::West => {
                    cart.direction = Direction::North;
                }
            }
        }
        '/' => {
            match cart.direction {
                Direction::North => {
                    cart.direction = Direction::East;
                }
                Direction::South => {
                    cart.direction = Direction::West;
                }
                Direction::East => {
                    cart.direction = Direction::North;
                }
                Direction::West => {
                    cart.direction = Direction::South;
                }
            }
        }
        '+' => {
            match cart.turns {
                Turn::Left => {
                    match cart.direction {
                        Direction::North => {
                            cart.direction = Direction::West;
                        }
                        Direction::South => {
                            cart.direction = Direction::East;
                        }
                        Direction::East => {
                            cart.direction = Direction::North;
                        }
                        Direction::West => {
                            cart.direction = Direction::South;
                        }
                    }
                    cart.turns = Turn::Straight;
                }
                Turn::Straight => {
                    cart.turns = Turn::Right;
                }
                Turn::Right => {
                    match cart.direction {
                        Direction::North => {
                            cart.direction = Direction::East;
                        }
                        Direction::South => {
                            cart.direction = Direction::West;
                        }
                        Direction::East => {
                            cart.direction = Direction::South;
                        }
                        Direction::West => {
                            cart.direction = Direction::North;
                        }
                    }
                    cart.turns = Turn::Left;
                }
            }
        }
        _ => {}
    }
    cart.position = next_position;
}

pub fn find_last_cart<'a, 'b>(empty_tracks: &'a str, carts: &'b mut Vec<Cart>) -> Option<&'b mut Cart> {
    let mut i = 0;

    let mut done = false;

    while !done {
        let done = carts
            .iter()
            .filter(|c| {
                !c.crashed
            }).count() == 1;

        println!("{}", done);
        if done {
            return
                carts.into_iter()
                    .find(|c| {
                        !c.crashed
                    });
        }

        let collision = move_carts(&empty_tracks, carts);

        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::Cart;

    const TEST_INPUT: &str = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";


    #[test]
    fn it_should_find_the_last_cart_left() {
        // Arrange
        let input = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
        let empty_track = crate::get_empty_tracks(input);
        let mut carts = crate::find_carts(input);

        // Act
        let result = crate::find_last_cart(&empty_track, &mut carts);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn it_should_compare_points() {
        // Arrange
        let point = crate::Point { x: 1, y: 10 };
        let point2 = crate::Point { x: 2, y: 10 };
        let point3 = crate::Point { x: 2, y: 11 };
        let point4 = crate::Point { x: 2, y: 13 };

        // Act
        let result = point < point2;
        let result2 = point3 < point4;
        let result3 = point3 == point3;

        // Assert
        assert_eq!(result, true);
        assert_eq!(result2, true);
        assert_eq!(result3, true);
    }

    #[test]
    fn it_should_get_width() {
        // Arrange
        let input = TEST_INPUT;

        // Act
        let width = crate::get_width(input);

        // Assert
        assert_eq!(width, 13);
    }

    #[test]
    fn it_should_get_char_at_coords() {
        // Arrange
        let input = TEST_INPUT;

        // Act
        let c = crate::find_char_in_string(input, 0, 0);
        let c2 = crate::find_char_in_string(input, 7, 2);

        // Assert
        assert_eq!(c, '/');
        assert_eq!(c2, '+');
    }

    #[test]
    fn it_should_find_carts() {
        // Arrange
        let input = TEST_INPUT;

        // Act
        let carts = crate::find_carts(input);

        // Assert
        assert_eq!(carts, vec![
            Cart {
                direction: crate::Direction::East,
                turns: crate::Turn::Left,
                position: crate::Point {
                    x: 2,
                    y: 0,
                },
                crashed: false,
            },
            Cart {
                direction: crate::Direction::South,
                turns: crate::Turn::Left,
                position: crate::Point {
                    x: 9,
                    y: 3,
                },
                crashed: false,
            }
        ]);
    }

    #[test]
    fn it_should_return_an_empty_track() {
        // Arrange
        let input = TEST_INPUT;
        let expected_tracks = r"/---\
|   |  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \-+--/
  \------/   ";

        // Act
        let empty_track = crate::get_empty_tracks(input);

        // Assert
        assert_eq!(empty_track, expected_tracks);
    }

    #[test]
    fn it_should_find_collisions() {
        // Arrange
        let input = TEST_INPUT;
        let empty_track = crate::get_empty_tracks(input);
        let mut carts = crate::find_carts(input);

        // Act
        let mut result = None;
        while result == None {
            result = crate::move_carts(&empty_track, &mut carts);
        }

        // Assert
        assert_eq!(result, Some(crate::Point { x: 7, y: 3 }));
    }


    #[test]
    fn it_should_change_direction_on_corners() {
        // Arrange
        let input = r"/---\
|   |  /----\
| /-+--+-\  |
| | |  | |  v
\-+-/  \-+--/
  \------/   ";

        let empty_track = crate::get_empty_tracks(input);
        let mut carts = crate::find_carts(input);
        let cart = carts.get_mut(0).unwrap();
        let expected_cart = crate::Cart {
            direction: crate::Direction::West,
            turns: crate::Turn::Left,
            position: crate::Point {
                x: 12,
                y: 4,
            },
            crashed: false,
        };

        // Act
        crate::update_cart(&empty_track, cart);

        // Assert
        assert_eq!(*cart, expected_cart);
    }

    #[test]
    fn it_should_change_direction_on_crossroads() {
        // Arrange
        let input = r"/---\
|   |  /----\
| /-+->+-\  |
| | |  | |  |
\-+-/  \-+--/
  \------/   ";

        let empty_track = crate::get_empty_tracks(input);
        let mut carts = crate::find_carts(input);
        let cart = carts.get_mut(0).unwrap();
        let expected_cart = crate::Cart {
            direction: crate::Direction::North,
            turns: crate::Turn::Straight,
            position: crate::Point {
                x: 7,
                y: 2,
            },
            crashed: false,
        };

        // Act
        crate::update_cart(&empty_track, cart);

        // Assert
        assert_eq!(*cart, expected_cart);
    }
}
