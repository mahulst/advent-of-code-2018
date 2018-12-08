use std::str::FromStr;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::atomic::{self, AtomicUsize};
use std::collections::HashSet;

extern crate uuid;
extern crate regex;

static OBJECT_COUNTER: AtomicUsize = atomic::ATOMIC_USIZE_INIT;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub id: usize,
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        let id = OBJECT_COUNTER.fetch_add(1, atomic::Ordering::SeqCst);

        Point { x, y, id }
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(input: &str) -> Result<Point, ()> {
        let re =
            Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();

        match re.captures(input) {
            Some(caps) => Ok(Point::new(
                caps["x"].parse().unwrap(),
                caps["y"].parse().unwrap(),
            )),
            None => Err(())
        }
    }
}

pub type Area = Vec<Vec<Option<usize>>>;

pub fn get_infinite_areas(areas: &Area) -> HashSet<&usize> {
    let mut edges = HashSet::new();

    // First row
    areas
        .first()
        .unwrap()
        .iter()
        .for_each(|p| {
            match p {
                Some(id) => {
                    edges.insert(id);
                }
                None => {}
            };
        });

    // lLast row
    areas
        .last()
        .unwrap()
        .iter()
        .for_each(|p| {
            match p {
                Some(id) => {
                    edges.insert(id);
                }
                None => {}
            };
        });

    // Row edges
    areas.iter().for_each(|row| {
        match row.first().unwrap() {
            Some(id) => {
                edges.insert(id);
            }
            None => {}
        };
        match row.last().unwrap() {
            Some(id) => {
                edges.insert(id);
            }
            None => {}
        };
    });

    edges
}

pub fn print_area(area: &Area) {
    for x in area {
        for y in x {
            match y {
                Some(id) => {
                    print!("{}", id);
                }
                None => print!("x")
            }
        }
        println!("");
    }
}

pub fn count_areas(area: &Area) -> HashMap<&usize, i32> {
    let mut map = HashMap::new();
    let infinite_areas = get_infinite_areas(&area);
    area
        .iter()
        .for_each(|vec| {
            vec
                .iter()
                .for_each(|opt| {
                    match opt {
                        Some(id) => {
                            let amount = map.entry(id).or_insert(0);
                            *amount += 1;
                        }
                        None => {}
                    }
                });
        });

    map
        .into_iter()
        .filter(|(id, _size)| {
            !infinite_areas.contains(id)
        }).collect()
}

pub fn get_sum_of_distances(points: &Vec<Point>, point: &Point) -> i32 {
    points
        .iter()
        .map(|p| get_distance(p, point))
        .sum()
}

pub fn get_area_of_distances(points: &Vec<Point>) -> Vec<i32> {
    let (width, height) = get_width_height(points);

    let mut region = Vec::new();
    for y in 0..height + 1 {
        for x in 0..width + 2 {
            let point = &Point::new(x, y);
            let distance_sum = get_sum_of_distances(points, point);
            region.push(distance_sum);
        }
    }

    region
}

fn get_width_height(points: &Vec<Point>) -> (i32, i32) {
    points
        .iter()
        .fold(
            (0, 0),
            |(width, height), p| (width.max(p.x), height.max(p.y)),
        )
}

pub fn get_area(points: &Vec<Point>) -> Area {
    let (width, height) = get_width_height(points);

    let mut area: Area = Vec::new();
    for y in 0..height + 1 {
        let mut row: Vec<Option<usize>> = Vec::new();

        for x in 0..width + 2 {
            match find_closes_point(&Point::new(x, y), points) {
                Some(p) => row.push(Some(p.id)),
                None => row.push(None)
            }
        }
        area.push(row);
    }

    area
}

pub fn find_closes_point<'a>(point: &'a Point, points: &'a Vec<Point>) -> Option<&'a Point> {
    let closest = points
        .iter()
        .fold((999, None), |closest, p| {
            let distance = get_distance(p, point);

            match closest.0.cmp(&distance) {
                Ordering::Less => closest,
                Ordering::Greater => (distance, Some(p)),
                Ordering::Equal => (distance, None)
            }
        });

    closest.1
}

pub fn get_distance(point: &Point, point2: &Point) -> i32 {
    (point.x - point2.x).abs() + (point.y - point2.y).abs()
}

#[cfg(test)]
mod tests {
    use crate::Point;
    use crate::get_distance;
    use crate::find_closes_point;
    use crate::count_areas;
    use crate::get_area;

    #[test]
    fn it_should_count_areas() {
        // Arrange
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9)
        ];

        // Act
        let area = get_area(&points);
        let result = count_areas(&area);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn it_should_find_closes_point() {
        // Arrange
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9)
        ];
        let point = Point::new(5, 2);

        // Act
        let result = find_closes_point(&point, &points).unwrap();

        // Assert
        assert_eq!(result, &points[4]);
    }

    #[test]
    fn it_should_not_find_a_point_when_multiple_are_closest() {
        // Arrange
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9)
        ];
        let point = Point::new(1, 4);

        // Act
        let result = find_closes_point(&point, &points);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn it_should_calculate_distance_to_point() {
        // Arrange
        let point = Point::new(1, 2);
        let point2 = Point::new(3, 7);

        // Act
        let result = get_distance(&point, &point2);

        // Assert
        assert_eq!(result, 7);
    }

    #[test]
    fn it_should_parse_input() {
        // Arrange
        let input = "300, 201";

        // Act
        let result: Point = input.parse().unwrap();

        // Assert
        assert_eq!(result.x, 300);
        assert_eq!(result.y, 201);
    }
}
