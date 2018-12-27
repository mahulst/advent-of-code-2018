use std::usize;
use std::cmp::Ordering;
use rayon;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt;
use std::thread;
use core::time;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<Square>,
    entities: Vec<EntityStats>,
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub struct EntityStats {
    pub health: i32,
    pub race: Race,
    pub position: Point,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Race {
    Goblin,
    Elf,
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Square {
    Wall,
    Empty,
}

#[derive(Copy, Clone, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Debug for EntityStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} at ({},{}) ({} health)", self.race, self.position.x, self.position.y, self.health)
    }
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

pub fn get_from_map(map: &Map, x: usize, y: usize) -> &Square {
    map.data.get(y * map.width + x).expect("Unable to get square from map")
}

pub fn get_position_from_map<'a, 'b>(map: &'a Map, point: &'b Point) -> &'a Square {
    get_from_map(map, point.x, point.y)
}

pub fn get_neighbours<'a>(map: &'a Map, point: &'a Point) -> Vec<Point> {
    let mut points = vec![];
    if point.x > 0 {
        points.push(Point { x: point.x - 1, y: point.y });
    }
    if point.x < map.width {
        points.push(Point { x: point.x + 1, y: point.y });
    }

    if point.y < map.height {
        points.push(Point { x: point.x, y: point.y + 1 });
    }

    if point.y > 0 {
        points.push(Point { x: point.x, y: point.y - 1 });
    }

    points
}

pub fn find_distances(map: &Map, point: &Point) -> Vec<Option<usize>> {
    let mut distances = vec![None; map.width * map.height];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit: VecDeque<(Point, usize)> = VecDeque::new();
    let mut to_visit_set: HashSet<(Point)> = HashSet::new();

    distances[point.x + point.y * map.width] = Some(0);
    visited.insert(*point);

    for neighbour in get_neighbours(map, point) {
        let is_occupied = map.entities.iter().find(|e| {
            e.position == neighbour && e.health > 0
        }).is_some();
        //todo: can being surrounded by all of the same race block moving?
        if !is_occupied {
            to_visit.push_front((neighbour.clone(), 1));
            to_visit_set.insert(neighbour.clone());
        } else {
            visited.insert(neighbour);
        }
    }

    while !to_visit.is_empty() {
        let (point, distance) = to_visit.pop_back().unwrap();
        visited.insert(point);
        match get_position_from_map(map, &point) {
            Square::Empty => {
                distances[point.x + point.y * map.width] = Some(distance);
                for neighbour in get_neighbours(map, &point) {
                    if !visited.contains(&neighbour) && !to_visit_set.contains(&neighbour) {
                        let is_occupied = map.entities.iter().find(|e| {
                            e.position == neighbour && e.health > 0
                        }).is_some();

                        if !is_occupied {
                            to_visit.push_front((neighbour.clone(), distance + 1));
                            to_visit_set.insert(neighbour.clone());
                        } else {
                            visited.insert(neighbour);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    distances
}

pub fn find_destination(map: &Map, entity: &EntityStats) -> Option<Point> {
    if !has_to_move(map, entity) {
        return None;
    }

    let targets = map.entities.iter().filter(|e| {
        e.race != entity.race && e.health > 0
    });
    let distances = find_distances(map, &entity.position);

    let mut spaces_next_to_target = vec![];

    targets.for_each(|entity| {
        let neighbours = get_neighbours(map, &entity.position);

        spaces_next_to_target.extend_from_slice(&neighbours);
    });


    let mut valid_spaces: Vec<(usize, Point)> = spaces_next_to_target.iter().filter_map(|p| {
        let distance: Option<usize> = *distances.get(p.x + p.y * map.width).expect("failed to get from map");

        let result = distance.and_then(|d| {
            Some((d, *p))
        });

        result
    }).collect();

    valid_spaces.sort_by(|a, b| {
        let order = match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.1.cmp(&b.1),
        };

        order
    });

    valid_spaces.first().and_then(|p| {
        Some(p.1)
    })
}

pub fn parse_map(input: &str) -> Map {
    let mut data = Vec::new();
    let mut entities = Vec::new();
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().len();

    let mut y = 0;

    input
        .lines()
        .for_each(|line| {
            let mut x = 0;
            line.chars().for_each(|c| {
                match c {
                    '#' => {
                        data.push(Square::Wall);
                    }
                    '.' => {
                        data.push(Square::Empty);
                    }
                    'E' => {
                        let elf = EntityStats {
                            health: 200,
                            race: Race::Elf,
                            position: Point { x, y },
                        };
                        entities.push(elf);
                        data.push(Square::Empty);
                    }
                    'G' => {
                        let goblin = EntityStats {
                            health: 200,
                            race: Race::Goblin,
                            position: Point { x, y },
                        };
                        entities.push(goblin);
                        data.push(Square::Empty);
                    }
                    _ => panic!("Unexpected map token")
                }
                x += 1;
            });
            y += 1;
        });

    Map { width, height, data, entities }
}

pub fn has_to_move(map: &Map, entity: &EntityStats) -> bool {
    let neighbours = get_neighbours(map, &entity.position);

    neighbours.into_iter().find(|p| {
        let found_entity_on_neighbour = map.entities.iter().find(|e| {
            entity.race != e.race && *p == e.position && e.health > 0
        }).is_some();

        found_entity_on_neighbour
    }).is_none()
}

pub fn find_next_step(map: &Map, origin: &Point, destination: &Point) -> Point {
    let distances = find_distances(map, destination);

    let mut neighbours: Vec<(usize, Point)> = get_neighbours(map, origin).iter().filter_map(|p| {
        let distance: Option<usize> = *distances.get(p.x + p.y * map.width).expect("Can't unwrap map position");
        distance.and_then(|d| {
            Some((d, *p))
        })
    }).collect();

    neighbours.sort_by(|a, b| {
        let order = match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.1.cmp(&b.1),
        };

        order
    });
    neighbours.first().and_then(|p| {
        Some(p.1)
    }).unwrap()
}

pub fn update_position(map: &Map, entity: &mut EntityStats) {
    if let Some(destination) = find_destination(map, entity) {
        let next_position = find_next_step(map, &entity.position, &destination);

        entity.position = next_position;
    } else {}
}

pub fn attack<'a, 'b>(map: &'a mut Map, entity: &'b EntityStats) {
    let neighbours = get_neighbours(map, &entity.position);

    let mut close_enemies: Vec<&mut EntityStats> = map.entities.iter_mut().filter(|e| {
        if e.race != entity.race && e.health > 0 {
            neighbours.iter().find(|n| *n == &e.position).is_some()
        } else {
            return false;
        }
    }).collect();


    close_enemies.sort_by(|a, b| {
        match a.health.cmp(&b.health) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.position.cmp(&b.position),
        }
    });

    if let Some(mut enemy) = close_enemies.get_mut(0) {
        if enemy.race == Race::Goblin {
            enemy.health -= 20;
        } else {
            enemy.health -= 3;
        }

        if enemy.health <= 0 && enemy.race == Race::Elf {
            panic!("Elf died");
        }
    };
}

pub fn simulate_battle(map: Map) -> i32 {
    let mut map = map;
    let mut rounds = 0;
    loop {
        map.entities.sort_by(|a, b| {
            a.position.cmp(&b.position)
        });

        for i in 0..map.entities.len() {
            let (elves, goblins): (Vec<EntityStats>, Vec<EntityStats>) = map.entities
                .iter()
                .filter(|e| e.health > 0)
                .partition(|e| e.race == Race::Elf);

            if elves.len() == 0 {
                let result = goblins.iter().fold(0, |acc, e| {
                    e.health + acc
                });

                return rounds * result;
            } else if goblins.len() == 0 {
                let result = elves.iter().fold(0, |acc, e| {
                    e.health + acc
                });

                return rounds * result;
            }
            let mut entity: Vec<EntityStats> = map.entities.splice(i..i + 1, vec![]).collect();
            let entity = entity.get_mut(0).unwrap();
            if entity.health > 0 {
                crate::update_position(&map, entity);
                crate::attack(&mut map, entity);
            }
            map.entities.insert(i, *entity);

            // Slow down steps for animation
//            let ten_millis = time::Duration::from_millis(10);
//            thread::sleep(ten_millis);

        }
        // clear terminal for cool animation
//        print!("{}[2J", 27 as char);
//        print_map(&map);

        rounds += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::Map;
    use crate::Point;
    use crate::Square::{Wall, Empty};
    use crate::EntityStats;
    use crate::Race::{Elf, Goblin};

    #[test]
    fn it_should_parse_input() {
        // Arrange
        let input = r"####
#EG#
#..#";

        // Act
        let result = crate::parse_map(input);

        // Assert
        assert_eq!(result, Map {
            width: 4,
            height: 3,
            data: vec![
                Wall,
                Wall,
                Wall,
                Wall,
                Wall,
                Empty,
                Empty,
                Wall,
                Wall,
                Empty,
                Empty,
                Wall
            ],
            entities: vec![
                EntityStats {
                    health: 200,
                    race: Elf,
                    position: Point {
                        x: 1,
                        y: 1,
                    },
                },
                EntityStats {
                    health: 200,
                    race: Goblin,
                    position: Point {
                        x: 2,
                        y: 1,
                    },
                }
            ],
        });
    }

    #[test]
    fn it_should_find_distances() {
        // Arrange
        let input = r"#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let map = crate::parse_map(input);
        let entity = map.entities.get(0).expect("Could not get entity");

        // Act
        let possible_locations = crate::find_distances(&map, &entity.position);

        // Assert
        assert_eq!(possible_locations, vec![
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(0),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            None,
            None,
            Some(2),
            None,
            Some(4),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        ]);
    }

    #[test]
    fn it_should_find_the_closest_target() {
        // Arrange
        // Arrange
        let input = r"#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let map = crate::parse_map(input);
        let entity = map.entities.get(0).expect("Could not get entity");

        // Act
        let target = crate::find_destination(&map, entity);

        assert_eq!(target, Some(Point { x: 3, y: 1 }));
    }

    #[test]
    fn it_should_find_the_next_step() {
        // Arrange
        // Arrange
        let input = r"#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let map = crate::parse_map(input);
        let entity = map.entities.get(0).expect("Could not get entity");


        // Act
        let target = crate::find_next_step(&map, &entity.position, &Point { x: 3, y: 1 });

        assert_eq!(target, Point { x: 2, y: 1 });
    }

    #[test]
    fn it_should_find_the_next_step2() {
        // Arrange
        // Arrange
        let input = r"#########
#.......#
#..GGG..#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########";
        let map = crate::parse_map(input);
        let entity = map.entities.get(3).expect("Could not get entity");


        // Act
        let destination = Point { x: 3, y: 3 };
        let target = crate::find_next_step(&map, &entity.position, &destination);

        assert_eq!(target, Point { x: 3, y: 3 });
    }

    #[test]
    fn it_should_make_all_entities_walk() {
        // Arrange
        let input = r"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";
        let mut map = crate::parse_map(input);

        // Act
        for _ in 0..3 {
            for i in 0..map.entities.len() {
                let mut entity: Vec<EntityStats> = map.entities.splice(i..i + 1, vec![]).collect();
                let entity = entity.get_mut(0).unwrap();
                crate::update_position(&map, entity);
                map.entities.insert(i, *entity);
            }
        }

        // Assert
        assert_eq!(map.entities, vec![
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 3, y: 2 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 4, y: 2 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 5, y: 2 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 3, y: 3 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Elf,
                position: Point { x: 4, y: 3 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 5, y: 3 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 1, y: 4 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 4, y: 4 },
            },
            EntityStats {
                health: 200,
                race: crate::Race::Goblin,
                position: Point { x: 7, y: 5 },
            }
        ]);
    }

    #[test]
    fn it_should_attack() {
        // Arrange
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut map = crate::parse_map(input);

        // Act
        let mut entity: Vec<EntityStats> = map.entities.splice(1..2, vec![]).collect();
        let entity = entity.get_mut(0).unwrap();
        crate::update_position(&map, entity);
        crate::attack(&mut map, entity);
        map.entities.insert(1, *entity);

        // Assert
        assert_eq!(map.entities.get(2).unwrap().health, 197);
    }

    #[test]
    fn it_should_simulate_battle() {
        // Arrange
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let mut map = crate::parse_map(input);
        let turns = 47;

        // Act
        let result = crate::simulate_battle(map);

        // Assert
        assert_eq!(result, 27730);
    }

    #[test]
    fn it_should_simulate_battle2() {
        // Arrange
        let input = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        let mut map = crate::parse_map(input);
        let turns = 47;

        // Act
        let result = crate::simulate_battle(map);

        // Assert
        assert_eq!(result, 36334);
    }
}

fn print_map(map: &Map) {
    for y in 0..map.height {
        for x in 0..map.width {
            match map.data.get(x + y * map.width).unwrap() {
                Square::Empty => {
                    let point = Point { x, y };
                    let entity = map.entities.iter().find(|e| {
                        e.position == point && e.health > 0
                    });
                    if let Some(e) = entity {
                        let letter = if e.race == crate::Race::Elf {
                            "E"
                        } else {
                            "G"
                        };
                        if e.health > 0 {
                            print!("{}", letter);
                        } else {
                            print!(".");
                        }
                    } else {
                        print!(".")
                    }
                }
                Square::Wall => print!("#")
            }
        }
        println!("");
    }
}