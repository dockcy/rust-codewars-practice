// replace
fn disemvowel(s: &str) -> String {
    let vowels = vec!["a", "e", "i", "o", "u", "y"];
    // s.chars().filter(|a| !vowels.contains(&&(a.to_lowercase().to_string())[..])).collect();
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

/// This time no story, no theory. The examples below show you how to write function accum:
///
/// Examples:
/// accum("abcd") -> "A-Bb-Ccc-Dddd"
/// accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
/// accum("cwAt") -> "C-Ww-Aaa-Tttt"
fn accum(s: &str) -> String {
    let mut i = 1;
    s.chars()
        .map(|c| {
            let mut word = vec![];
            for j in 0..i {
                if j == 0 {
                    word.push(c.to_ascii_uppercase());
                } else {
                    word.push(c.to_ascii_lowercase());
                }
            }
            i += 1;
            word.iter().collect()
        })
        .collect::<Vec<String>>()
        .join("-")
}

#[cfg(test)]
mod date20211129_tests {
    use super::*;
    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }

    #[test]
    fn basic_tests() {
        assert_eq!(
            accum("ZpglnRxqenU"),
            "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("NyffsGeyylB"),
            "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
        );
        assert_eq!(
            accum("MjtkuBovqrU"),
            "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
        );
        assert_eq!(
            accum("EvidjUnokmM"),
            "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
        );
        assert_eq!(
            accum("HbideVbxncC"),
            "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
        );
    }
}
