use std::iter::FromIterator;

struct Circle {
    values: Vec<usize>,
}

impl Circle {
    fn new(length: usize) -> Circle {
        Circle {
            values: Vec::from_iter(0..length),
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn reverse_part(&mut self, index: usize, length: usize) {
        // Normally we could use slice's own reverse() method, but it doesn't
        // exactly deal well with the circular nature of this wrapping thing.
        if length <= 1 {
            // Nothing to do
            return;
        }

        let len = self.values.len();
        let mut a = index;
        let mut b = index + length - 1;
        while a < b {
            self.values.swap(a % len, b % len);
            a += 1;
            b -= 1;
        }
    }

}

struct KnotHasher {
    position: usize,
    skip_size: usize,
}

impl KnotHasher {
    fn new() -> KnotHasher {
        KnotHasher {
            position: 0,
            skip_size: 0,
        }
    }

    fn hash(&mut self, circle: &mut Circle, input: &[u8]) -> String {
        let fixed = [17, 31, 73, 47, 23];
        let lengths: Vec<usize> = input.iter()
            .chain(fixed.iter())
            .map(|&n| n as usize)
            .collect();

        for _ in 0..64 {
            self.tie_knots(circle, &lengths[..]);
        }

        self.get_dense_hash(circle)
    }

    fn tie_knots(&mut self, circle: &mut Circle, lengths: &[usize]) {
        for &length in lengths.iter() {
            circle.reverse_part(self.position, length);
            self.position = (self.position + length + self.skip_size) % circle.len();
            self.skip_size += 1;
        }
    }

    fn get_dense_hash(&self, circle: &Circle) -> String {
        circle.values[..].chunks(16)
            .map(|chunk| chunk.iter().fold(0, |a, &b| a ^ b))
            .fold(String::new(), |mut s, x| {
                s.push_str(format!("{:02x}", x).as_str());
                s
            })
    }
}

fn main() {
    let input = [14, 58, 0, 116, 179, 16, 1, 104, 2, 254, 167, 86, 255, 55, 122, 244];
    let input_bytes = b"14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244";

    let mut circle = Circle::new(256);
    let mut hasher = KnotHasher::new();
    hasher.tie_knots(&mut circle, &input);
    let part1 = circle.values[0] * circle.values[1];
    println!("Part 1 = {}", part1);

    let mut circle = Circle::new(256);
    let mut hasher = KnotHasher::new();
    let part2 = hasher.hash(&mut circle, &input_bytes[..]);
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tie_knots() {
        let lengths = [3, 4, 1, 5];
        let mut circle = Circle::new(5);
        let mut hasher = KnotHasher::new();
        hasher.tie_knots(&mut circle, &lengths);
        assert_eq!(circle.values[0], 3);
        assert_eq!(circle.values[1], 4);
    }
}
