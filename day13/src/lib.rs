#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, PartialEq)]
pub struct Track {
    pub top_left: Point,
    pub bottom_right: Point,
    pub cart: Option<Cart>,
}

#[derive(Debug, PartialEq)]
pub struct Cart {
    pub direction: Direction,
    pub position: i32,
}

pub fn find_top_left_corners(input: &str) -> Vec<Point> {
    let mut y = 0;
    let mut list = Vec::new();

    input.lines()
        .for_each(|l| {
            l
                .match_indices("/-")
                .for_each(|(x, _)| {
                    list.push(Point { x: x as i32, y })
                });
            l
                .match_indices("/+")
                .for_each(|(x, _)| {
                    list.push(Point { x: x as i32, y })
                });

            y += 1;
        });

    list
}


fn get_closing_right_corner(input: &str, top_left: &Point) -> Option<Point> {
    let mut x = -1;
    let mut y = -1;
    let mut result = None;
    input.lines().for_each(|l| {
        y += 1;

        if y == top_left.y {
            l.chars().for_each(|c| {
                x += 1;

                if x > top_left.x {
                    if c == '\\' && result == None {
                        result = Some(Point { x, y });
                    }
                }
            })
        }
    });

    result
}

fn get_closing_bottom_right_corner(input: &str, top_right: &Point) -> Option<Point> {
    let mut y = -1;
    let mut result = None;

    input.lines().for_each(|l| {
        let mut x = -1;

        y += 1;
        if y >= top_right.y {
            l.chars().for_each(|c| {
                x += 1;
                if x == top_right.x {
                    if c == '/' && result == None {
                        result = Some(Point { x, y });
                    }
                }
            })
        }
    });

    result
}

pub fn find_bottom_right_corners(input: &str, top_lefts: Vec<Point>) -> Vec<Track> {
    top_lefts
        .iter()
        .map(|top_left| {
            let top_right = get_closing_right_corner(input, top_left).unwrap();

            let bottom_right = get_closing_bottom_right_corner(input, &top_right).unwrap();

            Track {
                top_left: *top_left,
                bottom_right,
                cart: None,
            }
        }).collect()
}

pub fn parse_tracks(input: &str) -> Vec<Track> {
    find_bottom_right_corners(input, find_top_left_corners(input))
}

fn find_cart(input: &str, track: &Track) -> Option<Cart> {
    let mut y = 0;
    let mut cart = None;
    input.lines().for_each(|l| {
        let mut x = -1;

        y += 1;

        l.chars().for_each(|c| {
            x += 1;

            let on_top_lane = y == track.top_left.y && x >= track.top_left.x && x <= track.bottom_right.x;
            let on_bottom_lane = y == track.bottom_right.y && x >= track.top_left.x && x <= track.bottom_right.x;
            let on_left_lane = y >= track.top_left.y && y <= track.bottom_right.y && x == track.top_left.x;
            let on_right_lane = y >= track.top_left.y && y <= track.bottom_right.y && x == track.bottom_right.x;

            let x_position_on_track = track.top_left.x - x;
            let y_position_on_track = y - track.top_left.y;
            let track_width = track.bottom_right.x - track.top_left.x;
            let track_height = track.bottom_right.y - track.top_left.y;

            match c {
                '>' => {
                    if on_top_lane {
                        let position = x_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::Clockwise
                        });
                    }

                    if on_bottom_lane {
                        let position = track_width + track_height + track_width - x_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::CounterClockwise
                        })
                    }
                },
                '<' => {
                    if on_top_lane {
                        let position = x_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::CounterClockwise
                        });
                    }

                    if on_bottom_lane {
                        let position = track_width + track_height + track_width - x_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::Clockwise
                        })
                    }
                },
                '^' => {
                    if on_left_lane {
                        let position = track_width * 2 + track_height + track_height - y_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::Clockwise
                        })
                    }
                    if on_right_lane {
                        let position = track_width + y_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::CounterClockwise
                        })
                    }
                },
                'v' => {

                    if on_left_lane {
                        let position = track_width * 2 + track_height + track_height - y_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::CounterClockwise
                        })
                    }
                    if on_right_lane {
                        let position = track_width + y_position_on_track;
                        cart = Some(Cart {
                            position,
                            direction: Direction::Clockwise
                        })
                    }
                },
                _ => {}
            }
        });

    });

    None
}

pub fn find_carts(input: &str, tracks: &mut Vec<Track>) {
    for track in tracks {
        let cart = find_cart(input, track);

        track.cart = cart;
        println!("{:#?}", track);

    }

}

#[cfg(test)]
mod tests {
    use crate::Track;
    use crate::Point;

    const TEST_INPUT: &str = r"/->-\
|   |  /+---\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/";

    #[test]
    fn it_should_parse_tracks() {
        // Act
        let result = crate::parse_tracks(TEST_INPUT);

        // Assert
        assert_eq!(result, vec![
            Track {
                top_left: Point {
                    x: 0,
                    y: 0,
                },
                bottom_right: Point {
                    x: 4,
                    y: 4,
                },
                cart: None,
            },
            Track {
                top_left: Point {
                    x: 7,
                    y: 1,
                },
                bottom_right: Point {
                    x: 12,
                    y: 4,
                },
                cart: None,
            },
            Track {
                top_left: Point {
                    x: 2,
                    y: 2,
                },
                bottom_right: Point {
                    x: 9,
                    y: 5,
                },
                cart: None,
            }
        ]);
    }

    #[test]
    fn it_should_find_carts_on_track() {
        // Arrange
        let mut tracks = crate::parse_tracks(TEST_INPUT);

        // Act
        crate::find_carts(TEST_INPUT, &mut tracks);
        // Assert

    }
}
