/// Count the number of Duplicates
/// Write a function that will return the count of distinct case-insensitive alphabetic characters and numeric digits that occur more than once in the input string. The input string can be assumed to contain only alphabets (both uppercase and lowercase) and numeric digits.
/// 
/// Example
/// "abcde" -> 0 # no characters repeats more than once
/// "aabbcde" -> 2 # 'a' and 'b'
/// "aabBcde" -> 2 # 'a' occurs twice and 'b' twice (`b` and `B`)
/// "indivisibility" -> 1 # 'i' occurs six times
/// "Indivisibilities" -> 2 # 'i' occurs seven times and 's' occurs twice
/// "aA11" -> 2 # 'a' and '1'
/// "ABBA" -> 2 # 'A' and 'B' each occur twice
fn count_duplicates(text: &str) -> u32 {
    let mut c_map = std::collections::HashMap::new();
    for mut c in text.chars() {
        c = c.to_ascii_lowercase();
        let num = c_map.entry(c).or_insert(0);
        *num += 1;
    }
    c_map.values().filter(|&&i| i > 1).count() as u32
}


/// Given a positive number n > 1 find the prime factor decomposition of n. The result will be a string with the following form :
/// 
///  "(p1**n1)(p2**n2)...(pk**nk)"
/// with the p(i) in increasing order and n(i) empty if n(i) is 1.
/// 
/// Example: n = 86240 should return "(2**5)(5)(7**2)(11)"
fn prime_factors(n: i64) -> String {
    let mut candidate = 2..;
    let mut prime_map = std::collections::BTreeMap::new();
    let mut _n = n;
    while _n > 1 {
        let x = candidate.next().unwrap();
        while _n % x == 0 {
            _n /= x;
            *prime_map.entry(x).or_insert(0) += 1;
        }
    }
    prime_map.iter().map(|(k,v)| match *v {
        1 => format!("({})", k),
        _ => format!("({}**{})",k,v),
    }).collect()
}

#[cfg(test)]
mod date20211215_tests{
    use super::*;
    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }
    
    #[test]
    fn basics_prime_factors() {
        
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17*17*93*677, "(3)(17**2)(31)(677)");
        
    }
}