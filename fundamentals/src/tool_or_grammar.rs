use std::ops::Add;

use regex::Regex;
// 四舍五入
fn litres(time: f64) -> i32 {
    let result: f64 = time * 0.5;
    let result_i32: i32 = result.floor() as i32;
    // let result_i32 : i32 = result.ceil() as i32;
    return result_i32;
}

// 正则表达式使用
fn abbrev_name(name: &str) -> String {
    Regex::new(r"^(.)[^ ]* (.).*$")
        .unwrap()
        .replace(name, "$1.$2")
        .to_string()
        .to_uppercase()
}

// replace
fn disemvowel(s: &str) -> String {
    s.replace(&['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'][..], "")
}

// 带下标的iterator : enumerate
fn accum(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_uppercase().to_string() + c.to_string().to_uppercase().repeat(i).as_str()
        })
        .collect::<Vec<String>>()
        .join("-")
}

// tuple的妙处
// 该题位于date/date_20211213.rs
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fib = (0, 1);
    while fib.0 * fib.1 < prod {
        fib = (fib.1, fib.0 + fib.1);
    }
    (fib.0, fib.1, fib.0 * fib.1 == prod)
}

// 质因数分解
fn prime_factors(n: i64) -> Vec<i64> {
    let mut candidate = 2..;
    let mut factors = vec![];
    let mut n = n;
    while n > 1 {
        let x = candidate.next().unwrap();
        while n % x == 0 {
            n /= x;
            factors.push(x);
        }
    }
    factors
}

/// 全组合的实现
/// list 原始数组
/// k 每一个组中得到数字个数
/// start_pos 每一次DFS从数组的start_pos开始
/// path 每一个组合
/// ans 所有的组合
pub fn combine(
    list: &Vec<i32>,
    k: usize,
    start_pos: usize,
    path: &mut Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
) {
    if path.len() == k {
        ans.push(path.clone());
        return;
    }
    for (i, &num) in list[start_pos..].to_vec().iter().enumerate() {
        path.push(num);
        combine(list, k, start_pos + 1, path, ans);
        path.pop();
    }
}

fn alphabet_position(text: &str) -> String {
    // Code here...
    let alphabet_map = ('a'..='z')
        .enumerate()
        .map(|(i, c)| (c, (i + 1).to_string()))
        .collect::<std::collections::HashMap<char, String>>();
    text.chars()
        .map(|c| match alphabet_map.get(&c.to_ascii_lowercase()) {
            Some(s) => s.clone() + " ",
            _ => "".to_string(),
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Greatest common Divisor
///
pub fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 || n == 0 {
        return 0;
    }
    let mut a = m;
    let mut b = n;
    if a > b {
       while a % b != 0 {
           let tmp = a;
           a = b;
           b = tmp % b;
       }
       b
    } else {
        while b % a != 0 {
            let tmp = b;
            b = a;
            a = tmp % a;
        }
        a
    }
}
