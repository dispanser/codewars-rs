// random thoughts
// - going over an array repeatedly is wasteful, especially towards the end,
//   when most numbers are crossed out, and especially for large n
// - instead, we'd need a "shrinking" array, which would allow us to go directly
//   k steps forward (cyclic).
// - OTOH, shrinking the array is very expensive
// - if my array has (remaining!) size n', and k > n', we can k -= n'
// - given a current position, and a list of all crossed-out things, we
//   can compute the next one to cross out based on how many crossed out
//   we're crossing and consequently how much extra we have to go to account
//   for that
//
// example:
// - n_0 = 0    // +3, because nothing "missing" yet
// - n_1 = 3X   // +3, because nothing missing from 3-6
// - n_2 = 6X   // +3%7, because 7, 1 are both still there
// - n_3 = 2X   // +5, because 3 and 6 are missing
// - n_4 = 7X   // k=0, because only three are left (???)
// - n_6 = 5X   //

// what data structures allow you to go forward the correct number of cells?
// - some sort of search? e.g., try k --> k' missing --> +k' --> k'' missing --> ...
// - basically
pub fn josephus_survivor(n: i32, k: i32) -> i32 {
    josephus_survivor_simple(n, k)
}

// initially, a simple solution?
fn josephus_survivor_simple(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut data: Vec<_> = (1..=n).collect();

    let mut remaining = n;
    let mut pos = n - 1;

    let mut last_removed: usize = 0;
    while remaining > 0 {
        // let mut k = 1.max(k % remaining); // no need to run circles
        let mut k = k; // no need to run circles
        while k > 0 {
            pos = (pos + 1) % n; // next step on the "ring"
            if data[pos] > 0 {
                // field still occupied?
                k -= 1; // count down
            }
        }
        data[pos] = 0; // clear field
        last_removed = pos + 1;
        remaining -= 1;
        println!(
            "removing: {}; remaining: {}; data: {:?}",
            last_removed, remaining, data
        );
    }
    last_removed as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(josephus_survivor(7, 3), 4);
        assert_eq!(josephus_survivor(11, 19), 10);
        assert_eq!(josephus_survivor(40, 3), 28);
        assert_eq!(josephus_survivor(14, 2), 13);
        assert_eq!(josephus_survivor(100, 1), 100);
        assert_eq!(josephus_survivor(1, 300), 1);
        assert_eq!(josephus_survivor(2, 300), 1);
        assert_eq!(josephus_survivor(5, 300), 1);
        assert_eq!(josephus_survivor(7, 300), 7);
        assert_eq!(josephus_survivor(300, 300), 265);
    }
}
