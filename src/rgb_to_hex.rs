fn nearest_u8(i: i32) -> u8 {
    i.min(255).max(0) as u8
    // std::cmp::min(255, std::cmp::max(0, i)) as u8
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        nearest_u8(r),
        nearest_u8(g),
        nearest_u8(b)
    )
}

// TODO: what's that, `assert_eq!` re-implemented?
macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected {
            panic!("Got: {}\nExpected: {}\n", $got, $expected);
        }
    };
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn test_rounding() {
        assert_eq!(nearest_u8(255), 255);
        assert_eq!(nearest_u8(256), 255);
        assert_eq!(nearest_u8(232435256), 255);
        assert_eq!(nearest_u8(-1235435), 0);
    }

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
