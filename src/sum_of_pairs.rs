// up to 10*10^6 elements in array
// - each number is from range -128 .. 127
// - checking if a solution exists is rather easy, but finding the first one...
// - idea: for each number, store how many times (we might need the same twice)
//   but also when first.
pub fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut counts: [i32; 256] = [0; 256];
    let mut relevant_numbers: Vec<_> = Vec::new();

    // store every number, but only up to two times. max size 512
    for v in ints {
        counts[(*v as i32 + 128) as usize] += 1;
        if counts[(*v as i32 + 128) as usize] <= 2 {
            relevant_numbers.push(*v);
        }
    }

    for j in 1..relevant_numbers.len() {
        for i in 0..j {
            if relevant_numbers[i] + relevant_numbers[j] == s {
                return Some((relevant_numbers[i], relevant_numbers[j]));
            }
        }
    }
    None
}

#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}
