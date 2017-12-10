fn main() {
    let input = include_str!("input.txt");

    let instructions = parse_instructions(input);

    let steps_1 = escape_maze_1(instructions.clone());
    println!("Part 1 = {}", steps_1);

    let steps_2 = escape_maze_2(instructions);
    println!("Part 2 = {}", steps_2);
}

fn parse_instructions(input: &str) -> Vec<isize> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let offset = match line.parse::<isize>() {
            Ok(n) => n,
            Err(_) => panic!("Unable to parse input line: {}", line),
        };
        instructions.push(offset);
    }
    instructions
}

fn escape_maze_1(mut instructions: Vec<isize>) -> usize {
    escape_maze_impl(instructions, |n| n + 1)
}

fn escape_maze_2(mut instructions: Vec<isize>) -> usize {
    escape_maze_impl(instructions, |n| if n >= 3 { n - 1 } else { n + 1 })
}

fn escape_maze_impl<F>(mut instructions: Vec<isize>, advance: F) -> usize
    where F: Fn(isize) -> isize
{
    let mut steps = 0usize;

    let mut index = 0isize;
    loop {
        let offset = match instructions.get_mut(index as usize) {
            Some(n) => n,
            None => break, // All done!
        };

        index += *offset;
        *offset = advance(*offset);
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = "0\n3\n0\n1\n-3";
        let instructions = parse_instructions(input);
        assert_eq!(escape_maze_1(instructions), 5);
    }

    #[test]
    fn test_2() {
        let input = "0\n3\n0\n1\n-3";
        let instructions = parse_instructions(input);
        assert_eq!(escape_maze_2(instructions), 10);
    }
}
