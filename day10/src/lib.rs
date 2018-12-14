use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;
use core::cmp;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Star {
    pub velocity: Point,
    pub position: Point,
}

pub fn input_to_stars(input: &str) -> HashMap<(i32, i32), Vec<Star>> {
    let mut map = HashMap::new();
    input
        .lines()
        .for_each(|l| {
            let star: Star = l.parse().unwrap();
            let list_of_stars = map.entry((star.position.x, star.position.y)).or_insert(vec![]);

            list_of_stars.push(star);
        });

    map
}

fn add(pos: &Point, vel: &Point) -> Point {
    Point { x: pos.x + vel.x, y: pos.y + vel.y }
}

pub fn tick_stars(stars: &HashMap<(i32, i32), Vec<Star>>) -> HashMap<(i32, i32), Vec<Star>> {
    let mut map = HashMap::new();
    stars
        .into_iter()
        .for_each(|(_pos, stars_on_this_pos)| {
            stars_on_this_pos.iter().for_each(|star| {
                let new_star = Star {
                    position: add(&star.position, &star.velocity),
                    velocity: star.velocity.clone(),
                };
                let list_of_stars = map.entry((new_star.position.x, new_star.position.y)).or_insert(vec![]);
                list_of_stars.push(new_star);
            });
        });

    map
}

pub fn get_bounds(stars: &HashMap<(i32, i32), Vec<Star>>) -> (i32, i32, i32, i32) {
    stars
        .into_iter()
        .fold((0, 0, 0, 0), |(left, right, top, bottom), (_, stars_on_this_pos)| {
            let new_bounds = stars_on_this_pos
                .into_iter()
                .fold((0, 0, 0, 0), |(left2, right2, top2, bottom2), star| {
                    (cmp::min(left2, star.position.x),
                     cmp::max(right2, star.position.x),
                     cmp::min(top2, star.position.y),
                     cmp::max(bottom2, star.position.y))
                });


            (cmp::min(left, new_bounds.0),
             cmp::max(right, new_bounds.1),
             cmp::min(top, new_bounds.2),
             cmp::max(bottom, new_bounds.3))
        })
}

pub fn draw_sky(stars: &HashMap<(i32, i32), Vec<Star>>) {
    let (left, right, top, bottom) = get_bounds(stars);

//
//    eprintln!("left = {:?}", left);
//    eprintln!("right = {:?}", right);
//    eprintln!("top = {:?}", top);
//    eprintln!("bottom = {:?}", bottom);


    for y in top..bottom+1 {
        for x in left..right+1 {
            match stars.get(&(x, y)) {
                Some(_) => {
                    print!("#");
                }
                None => print!(".")
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use crate::Star;
    use crate::Point;
    use crate::input_to_stars;
    use crate::get_bounds;
    use std::collections::HashMap;
    use crate::tick_stars;

    #[test]
    fn it_should_parse_input() {
        // Arrange
        let input = "position=< 9,  1> velocity=< 0,  2>";
        let position = Point { x: 9, y: 1 };
        let velocity = Point { x: 0, y: 2 };
        let expected_input = Star { position, velocity };

        // Act
        let result: Star = input.parse().unwrap();

        // Assert
        assert_eq!(result, expected_input);
    }

    #[test]
    fn it_should_parse_negative_input() {
        // Arrange
        let input = "position=< -9,  -1> velocity=< -0,  -2>";
        let position = Point { x: -9, y: -1 };
        let velocity = Point { x: -0, y: -2 };
        let expected_input = Star { position, velocity };

        // Act
        let result: Star = input.parse().unwrap();

        // Assert
        assert_eq!(result, expected_input);
    }

    #[test]
    fn it_should_get_outer_bounds_of_sky() {
        // Arrange
        let input = "position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>";

        // Act
        let stars = input_to_stars(&input);
        let bounds = get_bounds(&stars);

        // Assert
        assert_eq!(bounds, (-6, 15, -4, 11));
    }

    #[test]
    fn it_should_update_star_position() {
        // Arrange
        let star: Star = "position=< 9,  1> velocity=< 4,  -2>".parse().unwrap();
        let mut map = HashMap::new();
        map.insert((star.position.x, star.position.y), vec![star]);

        // Act
        let new_map = tick_stars(&map);

        // Assert
        let expected_star = Star {
            position: Point { x: 13, y: -1 },
            velocity: Point { x: 4, y: -2 },
        };
        let mut expected_map = HashMap::new();
        expected_map.insert(
            (expected_star.position.x, expected_star.position.y), vec![expected_star],
        );

        assert_eq!(new_map, expected_map);
    }
}

impl FromStr for Star {
    type Err = ();
    fn from_str(input: &str) -> Result<Star, ()> {
        let re =
            Regex::new(r"position=<\s*(?P<posx>-?\d*),\s*(?P<posy>-?\d*)> velocity=<\s*(?P<velx>-?\d*),\s*(?P<vely>-?\d*)>").unwrap();

        match re.captures(input) {
            Some(caps) => {
                let result = Star {
                    position: Point {
                        x: caps["posx"].parse().unwrap(),
                        y: caps["posy"].parse().unwrap(),
                    },
                    velocity: Point {
                        x: caps["velx"].parse().unwrap(),
                        y: caps["vely"].parse().unwrap(),
                    },
                };

                Ok(result)
            }
            None => Err(())
        }
    }
}