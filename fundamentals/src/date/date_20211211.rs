/// <<Reverse words>>
/// Complete the function that accepts a string parameter, and reverses each word in the string. All spaces in the string should be retained.
/// 
/// Examples
/// "This is an example!" ==> "sihT si na !elpmaxe"
/// "double  spaces"      ==> "elbuod  secaps"

fn reverse_words(str: &str) -> String {
    str.split(' ').map(|word| word.chars().rev().collect::<String>()).collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod date20211211_tests {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!("ih ih",reverse_words("hi hi"));
        assert_eq!("olleh",reverse_words("hello"));
        assert_eq!("tsuj eb ko",reverse_words("just be ok"));
    }
}