fn abbrev_name(name: &str) -> String {
    name.split(' ')
        .map(|a| a.chars().nth(0).unwrap().to_string().to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
}

/// Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).
///
/// Examples:
///
/// solution('abc', 'bc') // returns true
/// solution('abc', 'd') // returns false
fn solution(word: &str, ending: &str) -> bool {
    // word.ends_with(ending)
    if word.len() < ending.len() {
        return false;
    }
    let end_of_word = &word[word.len() - ending.len()..];
    end_of_word == ending
}

#[cfg(test)]
mod date20211128_tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
    }
}
