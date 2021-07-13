fn is_whitespace_only(input: &str) -> bool {
    input.chars().all(|c| c.is_whitespace())
}

/// the "submitted" solution (passing).
/// this implementation only works for ascii IIUC, because of
/// how it uses byte positions to create slices. If characters
/// occupy more than one byte, we panic:
/// panicked at 'byte index 1 is not a char boundary;
pub fn wave_ascii(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in 0..s.len() {
        let infix: &str = &s[i..(i + 1)].to_string().to_uppercase();

        if !is_whitespace_only(infix) {
            let prefix: &str = &s[0..i];
            let suffix: &str = &s[(i + 1)..s.len()];
            result.push(format!("{}{}{}", prefix, infix, suffix));
        }
    }
    result
}

/// utf-8 compliant.
pub fn wave(s: &str) -> Vec<String> {
    s.char_indices()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, c)| {
            let char_len = &c.len_utf8();
            format!("{}{}{}", &s[0..i], &c.to_uppercase(), &s[(i + char_len)..])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8() {
        let x: char = 'λ';
        assert_eq!(x.len_utf8(), 2);
        assert_eq!(wave("λ"), ["Λ"]);
        assert_eq!(wave("λσ"), ["Λσ", "λΣ"]);
    }

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);

        let expect = [
            "Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs",
            "codewarS",
        ];
        assert_eq!(wave("codewars"), expect);

        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);

        let expect = [
            "Two words",
            "tWo words",
            "twO words",
            "two Words",
            "two wOrds",
            "two woRds",
            "two worDs",
            "two wordS",
        ];
        assert_eq!(wave("two words"), expect);

        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}
