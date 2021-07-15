use std::collections::{HashMap, HashSet};

/// idea:
/// - we know that all letters only appear once in the solution
/// - a solution is possible from the given triplets
/// - every triplet consequently allows us to deduce before-after relationship
/// - make the partial ordering total: given a < b, for all c with c < a deduce c < b
/// - from the guarantees, this must yield a total ordering.
/// - unclear: could beocme very expensive because every new fact will lead to more facts
/// - alternative: establish partial ordering and then find the min, discard, and repeat.

pub fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut before: HashMap<char, HashSet<char>> = HashMap::new();
    let mut after: HashMap<char, HashSet<char>> = HashMap::new();

    triplets.iter().for_each(|t| {
        before.entry(t[0]).or_default().insert(t[1]);
        before.entry(t[1]).or_default().insert(t[2]);
        after.entry(t[1]).or_default().insert(t[0]);
        after.entry(t[2]).or_default().insert(t[1]);
    });
    let mut unplaced_chars: HashSet<char> = before
        .keys()
        .into_iter()
        .chain(after.keys().into_iter())
        .map(|c| *c)
        .collect();

    let mut solution = String::new();
    while !unplaced_chars.is_empty() {
        let head = *unplaced_chars
            .iter()
            .find(|&&c| after.entry(c).or_default().is_empty())
            .unwrap();
        unplaced_chars.remove(&head);
        solution.push(head);
        before.entry(head).or_default().iter().for_each(|&c| {
            after.entry(c).or_default().remove(&head);
        });
    }

    solution
}

#[test]
fn example_test() {
    assert_eq!(recover_secret(vec![]), "");
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}
