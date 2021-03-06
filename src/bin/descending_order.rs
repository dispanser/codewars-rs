// required for `sorted()` method on iterators
extern crate itertools;
use itertools::Itertools;

fn main() {}

pub fn descending_order(x: u64) -> u64 {
    let s: String = x.to_string().chars().sorted().rev().collect();
    s.parse().expect("should be a valid number")
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
