use std::collections::HashMap;
use regex::Regex;

extern crate regex;

const FABRIC_LENGTH: i32 = 1000;

#[derive(Debug)]
pub struct Claim {
    pub id: i32,
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

pub fn plot_square(fabric: &mut HashMap<i32, i32>, claim: &Claim) {
    for x_coord in claim.left..claim.right {
        for y_coord in claim.top..claim.bottom {
            let amount = fabric.entry(x_coord + (y_coord * FABRIC_LENGTH)).or_insert(0);

            *amount += 1;
        }
    }
}

pub fn count_overlap(fabric: &HashMap<i32, i32>, minimum: i32) -> usize {
    fabric.into_iter().filter(|&(_, value)| {
        *value >= minimum
    }).count()
}

pub fn parse_line(line: &str) -> Claim {
    let re = Regex::new(r"\#(\d+) @ (\d{1,3}),(\d{1,3}): (\d{1,3})x(\d{1,3})").unwrap();

    let caps = re.captures(line).unwrap();

    let id = caps.get(1).unwrap().as_str().parse().unwrap();
    let x = caps.get(2).unwrap().as_str().parse().unwrap();
    let y = caps.get(3).unwrap().as_str().parse().unwrap();
    let width = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    let height = caps.get(5).unwrap().as_str().parse::<i32>().unwrap();

    Claim {
        id,
        left: x,
        right: x + width,
        top: y,
        bottom: y + height,
    }
}

pub fn find_non_overlapping<'a>(claims: &'a Vec<Claim>, fabric: &'a HashMap<i32, i32>) -> Option<&'a Claim> {
    claims.iter().find(|claim| {
        let mut overlap = false;
        for x_coord in claim.left..claim.right {
            for y_coord in claim.top..claim.bottom {
                let index = x_coord + (y_coord * FABRIC_LENGTH);
                let count = fabric.get(&index).unwrap();
                overlap = overlap || *count != 1;
            }
        }
        return !overlap;
    })
}

#[cfg(test)]
mod tests {
    use plot_square;
    use std::collections::HashMap;
    use count_overlap;
    use parse_line;

    #[test]
    fn it_should_parse_a_line() {
        let line = "#1346 @ 700,889: 11x25";


        let claim = parse_line(line);

        assert_eq!(claim.left, 700);
        assert_eq!(claim.right, 700 + 11);
        assert_eq!(claim.top, 889);
        assert_eq!(claim.bottom, 889 + 25);
    }

    #[test]
    fn it_should_plot_square_on_fabric() {
        let mut fabric = HashMap::new();
        let claim = parse_line("#1 @ 10,10: 4x5");

        plot_square(&mut fabric, claim);
        let amount = count_overlap(&fabric, 1);

        assert_eq!(amount, 20);
    }

    #[test]
    fn it_should_plot_overlapping_square_on_fabric() {
        let mut fabric = HashMap::new();
        let claim1 = parse_line("#1 @ 1,3: 4x4");
        let claim2 = parse_line("#1 @ 3,1: 4x4");


        plot_square(&mut fabric, claim1);
        plot_square(&mut fabric, claim2);

        let amount = count_overlap(&fabric, 2);

        assert_eq!(amount, 4);
    }
}
