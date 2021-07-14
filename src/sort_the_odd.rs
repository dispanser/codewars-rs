/// sorting only the odd numbers, keeping the even ones in place
/// - go over the array once, putting the odds into a separate vec and
///   simultaneously marking the odd positions
/// - sort the separated odds
/// - fill them back in
pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds: Vec<i32> = arr.iter().filter(|x| **x % 2 == 1).map(|x| *x).collect();
    odds.sort();

    println!("{:?}", odds);
    let mut solution: Vec<i32> = vec![];

    let mut odd_index = 0;
    for &v in arr {
        if v % 2 == 0 {
            solution.push(v);
        } else {
            solution.push(odds[odd_index]);
            odd_index += 1;
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
