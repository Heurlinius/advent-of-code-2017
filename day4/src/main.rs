use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let mut number_valid1 = 0usize;
    let mut number_valid2 = 0usize;
    for line in input.lines() {
        if passphrase_valid(line) {
            number_valid1 += 1;
        }
        if passphrase_valid2(line) {
            number_valid2 += 1;
        }
    }
    println!("Part 1 = {}", number_valid1);
    println!("Part 2 = {}", number_valid2);
}

fn passphrase_valid(passphrase: &str) -> bool {
    let mut seen_words = HashSet::new();

    for word in passphrase.split_whitespace() {
        if !seen_words.insert(word) {
            return false;
        }
    }

    true
}

fn passphrase_valid2(passphrase: &str) -> bool {
    let mut seen_words = HashSet::new();

    for word in passphrase.split_whitespace() {
        let mut word: Vec<char> = word.chars().collect();
        word.sort();
        if !seen_words.insert(word) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert!(passphrase_valid("aa bb cc dd ee"));
    }

    #[test]
    fn test_2() {
        assert!(!passphrase_valid("aa bb cc dd aa"));
    }

    #[test]
    fn test_3() {
        assert!(passphrase_valid("aa bb cc dd aaa"));
    }

    #[test]
    fn test_4() {
        assert!(passphrase_valid2("abcde fghij"));
    }

    #[test]
    fn test_5() {
        assert!(!passphrase_valid2("abcde xyz ecdab"));
    }

    #[test]
    fn test_6() {
        assert!(passphrase_valid2("a ab abc abd abf abj"));
    }

    #[test]
    fn test_7() {
        assert!(passphrase_valid2("iiii oiii ooii oooi oooo"));
    }

    #[test]
    fn test_8() {
        assert!(!passphrase_valid2("oiii ioii iioi iiio"));
    }
}
