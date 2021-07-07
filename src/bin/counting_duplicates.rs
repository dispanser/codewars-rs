extern crate itertools;

use std::collections::HashMap;

fn main() {}

pub fn count_duplicates(test: &str) -> u32 {
    let mut counts: HashMap<_, _> = HashMap::new();
    for &byte in test.to_ascii_uppercase().as_bytes() {
        (*counts.entry(byte).or_insert(0)) += 1;
    }

    counts.values().filter(|&&x| x > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
