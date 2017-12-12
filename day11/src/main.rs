// With massive props to Amit Patel / Red Blob Games for this indispensable resource
// on hexagonal geometry: https://www.redblobgames.com/grids/hexagons
// All my <3 and respect.

// Cube coordinates ftw
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hex(isize, isize, isize);

impl Hex {
    fn origin() -> Hex {
        Hex(0, 0, 0)
    }

    fn offset(&self, dir: Direction) -> Hex {
        let offset = dir.get_offset();
        Hex(
            self.0 + offset.0,
            self.1 + offset.1,
            self.2 + offset.2
        )
    }

    fn distance_between(a: &Hex, b: &Hex) -> usize {
        let dx = (a.0 - b.0).abs();
        let dy = (a.1 - b.1).abs();
        let dz = (a.2 - b.2).abs();
        ((dx + dy + dz) / 2) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl Direction {
    fn get_offset(self) -> (isize, isize, isize) {
        match self {
            Direction::N  => (0, 1, -1),
            Direction::NE => (1, 0, -1),
            Direction::SE => (1, -1, 0),
            Direction::S  => (0, -1, 1),
            Direction::SW => (-1, 0, 1),
            Direction::NW => (-1, 1, 0),
        }
    }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input.split(",")
        .map(|dir| match dir {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => panic!("Invalid direction: {}", dir),
        })
        .collect()
}

fn calculate_distance(start: &Hex, directions: &[Direction], max_distance: &mut usize) -> usize {
    let mut position = start.clone();
    for &dir in directions.into_iter() {
        position = position.offset(dir);
        *max_distance = std::cmp::max(*max_distance, Hex::distance_between(&position, &start));
    }

    Hex::distance_between(&position, &start)
}

fn main() {
    let input = include_str!("input.txt");

    let directions = parse_directions(input);
    let start = Hex::origin();

    let mut part2 = 0usize;
    let part1 = calculate_distance(&start, &directions, &mut part2);
    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;
    // For brevity and/or because I'm lazy
    use Direction::*;

    #[test]
    fn test_distance_1() {
        let directions = [NE, NE, NE];
        let start = Hex::origin();
        let distance = calculate_distance(&start, &directions);
        assert_eq!(distance, 3);
    }

    #[test]
    fn test_distance_2() {
        let directions = [NE, NE, SW, SW];
        let start = Hex::origin();
        let distance = calculate_distance(&start, &directions);
        assert_eq!(distance, 0);
    }

    #[test]
    fn test_distance_3() {
        let directions = [NE, NE, S, S];
        let start = Hex::origin();
        let distance = calculate_distance(&start, &directions);
        assert_eq!(distance, 2);
    }

    #[test]
    fn test_distance_4() {
        let directions = [SE, SW, SE, SW, SW];
        let start = Hex::origin();
        let distance = calculate_distance(&start, &directions);
        assert_eq!(distance, 3);
    }
}
