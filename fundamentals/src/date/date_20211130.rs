use std::collections::HashSet;

/// Given a list of integers and a single sum value, return the first two values (parse from the left please) in order of appearance that add up to form the sum.
/// sum_pairs([0, 0, -2, 3], 2)
/// #  there are no pairs of values that can be added to produce 2.
/// == None/nil/undefined (Based on the language)
///
/// 0 5 8 7 3 5
/// sum_pairs([10, 5, 2, 3, 7, 5],         10)
/// #              ^-----------^   5 + 5 = 10, indices: 1, 5
/// #                    ^--^      3 + 7 = 10, indices: 3, 4 *
/// #  * entire pair is earlier, and therefore is the correct answer
/// == [3, 7]
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // your code
    let mut cache_set = HashSet::new();
    for i in ints {
        if !cache_set.contains(i) {
            cache_set.insert(s-i);
        } else {
            let res_tuple = (s-i,*i);
            return Some(res_tuple);
        }
    }
    return None
}

#[cfg(test)]
mod date20211130_tests {
    use super::*;
    
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
}
