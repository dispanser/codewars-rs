fn main() {}

pub fn nb_year(start_pop: i32, percent: f64, new_yearly: i32, threshold: i32) -> i32 {
    if start_pop >= threshold {
        0
    } else {
        let next_pop = start_pop + ((start_pop as f64 * (percent / 100.0)) as i32) + new_yearly;
        eprintln!("{} -> {}", start_pop, next_pop);
        1 + nb_year(next_pop, percent, new_yearly, threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1000, 2.0, 50, 1200, 3);
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
    }
}
