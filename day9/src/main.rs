fn visit_data(input: &str, visitor: &mut Visitor) {
    let mut depth = 0;
    let mut in_garbage = false;
    let mut escaped = false;
    let mut garbage_length = 0;

    for ch in input.chars() {
        match ch {
            '{' if !in_garbage => {
                depth += 1;
                visitor.begin_group(depth);
            },
            '}' if !in_garbage => {
                depth -= 1;
            },
            // Only occurs as a separator within groups, ignore it
            ',' if !in_garbage => (), 
            '<' if !in_garbage => {
                in_garbage = true;
            },
            '>' if in_garbage && !escaped => {
                in_garbage = false;
                visitor.add_garbage(garbage_length);
                garbage_length = 0;
            },
            '!' if in_garbage && !escaped => {
                escaped = true;
            },
            _ => if in_garbage {
                if !escaped {
                    garbage_length += 1;
                }
                escaped = false;
            } else {
                panic!("Invalid character in input: {}", ch);
            },
        }
    }
}

struct Visitor {
    score: usize,
    garbage: usize,
}

impl Visitor {
    fn new() -> Visitor {
        Visitor {
            score: 0,
            garbage: 0,
        }
    }

    fn begin_group(&mut self, depth: usize) {
        self.score += depth;
    }

    fn add_garbage(&mut self, length: usize) {
        self.garbage += length;
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut visitor = Visitor::new();
    visit_data(input, &mut visitor);
    println!("Part 1 = {}", visitor.score);
    println!("Part 2 = {}", visitor.garbage);
}
