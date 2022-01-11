use std::ascii::AsciiExt;

fn how_much(m: i32, n: i32) -> Vec<(String, String, String)> {
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    if m > n {
        max = m + 1;
        min = n;
    } else {
        max = n + 1;
        min = m;
    }
    (min..max)
        .filter(|&x| (x - 1) % 9 == 0 && (x - 2) % 7 == 0)
        .map(|x| {
            let mut ans: (String, String, String) =
                (String::from(""), String::from(""), String::from(""));
            ans.0 = format!("M: {}", x);
            ans.1 = format!("B: {}", x / 7);
            ans.2 = format!("C: {}", x / 9);
            ans
        })
        .collect::<Vec<(String, String, String)>>()
}

fn encrypt_this(text: &str) -> String {
    text.split(' ')
        .map(|word| {
            let mut it = word.chars();
            let capital = it.next().unwrap();
            let mut result_word = it.map(|c| c.to_string()).collect::<Vec<_>>();
            let len = result_word.len();
            if len == 0 {
                (capital as u8).to_string()
            } else {
                result_word.swap(0, len - 1);
                let res = format!("{}{}", capital as u8, result_word.join(""));
                res
        }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod date20220102_tests {
    use super::*;

    fn testing(m: i32, n: i32, exp: Vec<(&str, &str, &str)>) -> () {
        let ans: String = format!("{:?}", how_much(m, n));
        let sol: String = format!("{:?}", exp);
        assert_eq!(ans, sol)
    }
    #[test]
    fn basics_how_much() {
        testing(
            1,
            100,
            vec![("M: 37", "B: 5", "C: 4"), ("M: 100", "B: 14", "C: 11")],
        );
        testing(1000, 1100, vec![("M: 1045", "B: 149", "C: 116")]);
        testing(10000, 9950, vec![("M: 9991", "B: 1427", "C: 1110")]);
        testing(
            0,
            200,
            vec![
                ("M: 37", "B: 5", "C: 4"),
                ("M: 100", "B: 14", "C: 11"),
                ("M: 163", "B: 23", "C: 18"),
            ],
        );
        testing(2950, 2950, vec![]);
    }
    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(
            encrypt_this(&"A wise old owl lived in an oak"),
            "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string()
        );
        assert_eq!(
            encrypt_this(&"The more he saw the less he spoke"),
            "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string()
        );
        assert_eq!(
            encrypt_this(&"The less he spoke the more he heard"),
            "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string()
        );
        assert_eq!(
            encrypt_this(&"Why can we not all be like that wise old bird"),
            "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string()
        );
        assert_eq!(
            encrypt_this(&"Thank you Piotr for all your help"),
            "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string()
        );
    }
}
