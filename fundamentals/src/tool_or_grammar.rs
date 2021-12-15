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
    let mut factors = vec!();
    let mut n  = n;
    while n > 1 {
        let x = candidate.next().unwrap();
        while n % x == 0 {
            n /= x;
            factors.push(x);
        }
    }
    factors
}