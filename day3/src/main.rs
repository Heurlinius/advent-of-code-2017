use std::collections::HashMap;

fn main() {
    let input = 325489;

    let part1 = distance_between_squares(1, input);
    println!("Part 1 = {}", part1);

    let part2 = find_value(input);
    println!("Part 2 = {}", part2);
}

fn find_layer_size(square: usize) -> usize {
    // Given square number 14, the outermost "layer" of the spiral
    // needs to be 5x5 squares, like so:
    //   17  16  15  14  13
    //   18   5   4   3  12
    //   19   6   1   2  11
    //   20   7   8   9  10
    //   21  22  23  24  25
    // You could shave off squares 17 to 25, but it makes no difference,
    // since the distance from 14 to 1 is the same.
    // The solution is simple:
    //   Find the lowest odd number n where n^2 >= square.

    let sqrt_square = f64::sqrt(square as f64);
    sqrt_square.ceil() as usize | 1
}

fn distance_from_center(square: usize) -> (isize, isize) {
    if square == 1 {
        (0, 0)
    } else {
        let layer_size = find_layer_size(square) as isize;
        let square = square as isize;
        let prev_size = layer_size - 2;
        // The index of the first square in the current layer:
        // 1 for the first layer, then 2, 10, 26 ...
        let layer_start = prev_size * prev_size + 1;

        // Now we can calculate the edge that the target square lies on, and
        // its distance along that edge.
        let to_center = layer_size / 2;
        let edge_length = layer_size - 1;
        let layer_dist = square - layer_start;
        let (edge, edge_dist) = (layer_dist / edge_length, layer_dist % edge_length);
        match edge {
            // Right edge
            0 => (to_center, to_center - 1 - edge_dist),
            // Top edge
            1 => (to_center - 1 - edge_dist, -to_center),
            // Left edge
            2 => (-to_center, -to_center + 1 + edge_dist),
            // Bottom edge
            3 => (-to_center + 1 + edge_dist, to_center),
            // Error edge
            _ => panic!("Unexpected edge case: {}", edge),
        }
    }
}

fn distance_between_squares(a: usize, b: usize) -> usize {
    let (a_x, a_y) = distance_from_center(a);
    let (b_x, b_y) = distance_from_center(b);

    ((a_x - b_x).abs() + (a_y - b_y).abs()) as usize
}

fn find_value(limit: usize) -> usize {
    let mut values = HashMap::new();
    values.insert((0, 0), 1); // seed value
    let mut value = 0;
    let mut square = 2;
    loop {
        value = write_next_value(square, &mut values);
        if value > limit {
            break;
        }
        square += 1;
    }
    value
}

fn write_next_value(square: usize, grid: &mut HashMap<(isize, isize), usize>) -> usize {
    let coord = distance_from_center(square);
    let neighbour_offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut value = 0usize;
    for offset in neighbour_offsets.into_iter() {
        match grid.get(&(coord.0 + offset.0, coord.1 + offset.1)) {
            Some(&x) => { value += x; },
            None => { },
        }
    }
    grid.insert(coord, value);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_outer_size() {
        assert_eq!(find_layer_size(1), 1);
        assert_eq!(find_layer_size(5), 3);
        assert_eq!(find_layer_size(10), 5);
        assert_eq!(find_layer_size(32), 7);
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance_between_squares(1, 1), 0);
        assert_eq!(distance_between_squares(1, 12), 3);
        assert_eq!(distance_between_squares(1, 23), 2);
        assert_eq!(distance_between_squares(1, 1024), 31);
    }
}
