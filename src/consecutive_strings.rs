// manual loop that always chops of a value from the beginning of the "current" slice,
// adding a new one from the end, maintaining a sum for `k` elements.
pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k <= 0 || k > strarr.len() {
        return String::new();
    }
    let lengths: Vec<usize> = strarr.iter().map(|s| s.len()).collect();
    let n = lengths.len();

    let mut max_value: usize = lengths[0..k].iter().sum();
    let mut max_index = 0;

    let mut current_length = max_value;
    for i in 0..(n - k) {
        current_length = current_length + lengths[i + k] - lengths[i];
        if current_length > max_value {
            max_value = current_length;
            max_index = i + 1;
        }
    }

    strarr[max_index..(max_index + k)].concat()
}

// uses `windows(usize)` on `Vec` to walk over all the possible
// windows of length `k`. It's not as efficient, as we're always
// building the string and then compute its lenght, but I could
// imagine that `windows` is just as efficient as my manual loop.
pub fn longest_windowing(strarr: Vec<&str>, k: usize) -> String {
    if k <= 0 || k > strarr.len() {
        String::new()
    } else {
        strarr
            .windows(k)
            .map(|x| x.join(""))
            .rev()
            .max_by_key(|s| s.len())
            .expect("at least one result must be in there")
    }
}

// combine the idea of doing the "math" on the lengths of the
// vectors instead of doing any actual string manipulation, but
// still use the windowing.
pub fn longest_windowing_lengths(strarr: Vec<&str>, k: usize) -> String {
    if k <= 0 || k > strarr.len() {
        String::new()
    } else {
        let lengths: Vec<usize> = strarr.iter().map(|s| s.len()).collect();
        // problem: windows seems to only give us the slice, but not its
        // position
        let (index, _length): (usize, usize) = lengths
            .windows(k)
            .map(|ls| ls.into_iter().sum())
            .enumerate()
            .rev()
            .max_by_key(|t| t.1)
            .expect("there must be a result in there, because k <= len");
        strarr[index..(index + k)].concat()
    }
}

#[cfg(test)]
fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_windowing_lengths(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["a", "b", "c", "d"], 2, "ab");
    testing(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    testing(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    testing(vec![], 3, "");
    testing(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
