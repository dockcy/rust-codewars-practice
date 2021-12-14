
use std::collections::HashMap;

/// The goal of this exercise is to convert a string to a new string where each character in the new string is "(" if that character appears only once in the original string, or ")" if that character appears more than once in the original string. Ignore capitalization when determining if a character is a duplicate.
///
/// Examples
/// "din"      =>  "((("
/// "recede"   =>  "()()()"
/// "Success"  =>  ")())())"
/// "(( @"     =>  "))(("
///
fn duplicate_encode(word: &str) -> String {
    let mut record_map: HashMap<_, usize> = HashMap::new();
    let mut res_vec = vec![];
    let mut i = 0;
    for mut c in word.chars() {
        c = c.to_ascii_lowercase();
        if record_map.contains_key(&c) {
            // 若重复为第一次（之前没重复过）
            if *record_map.get(&c).unwrap() != usize::MAX {
                let pos =  *record_map.get(&c).unwrap();
                // 更新第一次的位置')'
                res_vec[pos] = ')';
                record_map.insert(c, usize::MAX);
            }
            // 更新该位置为')'
            res_vec.push(')');
        } else {
            record_map.insert(c, i );
            res_vec.push('(');
        }
        i += 1;
    }

    res_vec.iter().collect::<String>()
}

/// 优解如下
/// fn duplicate_encode(word: &str) -> String {
///     let mut enc = std::collections::HashMap::new();
///     for c in word.to_lowercase().chars() {
///       *enc.entry(c).or_insert(0) += 1;
///     }
///     word.to_lowercase().chars().map(|c| match *enc.get(&c).unwrap() {
///       1 => '(',
///       _ => ')'
///     }).collect()
///   }

#[cfg(test)]
mod date20211214_tests {
    use super::*;
    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
