fn score(input: &str) -> usize {
    input.bytes().map(|b| (b - 96) as usize).sum()
}

pub fn high(input: &str) -> &str {
    let (_score, word) = input
        // in contrast to `split(" ")`, gives `DoubleEndedIterator` (for `rev`)
        .split_ascii_whitespace()
        .rev()
        .map(|word| (score(word), word))
        .max_by_key(|score_and_word| score_and_word.0)
        .expect("non-empty");
    word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("abc"), 6);
        assert_eq!(score("cba"), 6);
        assert_eq!(score("zzzzzzzzzz"), 260);
    }

    #[test]
    fn test_basic() {
        assert_eq!(high("aaa b"), "aaa");
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");
        assert_eq!(high("massage yes massage yes massage"), "massage");
        assert_eq!(high("take two bintang and a dance please"), "bintang");
        assert_eq!(high("aa b"), "aa");
        assert_eq!(high("b aa"), "b");
        assert_eq!(high("bb d"), "bb");
        assert_eq!(high("d bb"), "d");
        assert_eq!(high("aaa b"), "aaa");
    }
}
