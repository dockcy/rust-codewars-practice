/// Title: Digital Root
/// Given n, take the sum of the digits of n
/// If that value has more than one digit, continue reducing in this way until a single-digit number is produced
/// The input will be a non-negative integer
/// 16  -->  1 + 6 = 7
/// 942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
/// 132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
/// 493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
fn digital_root(n: i64) -> i64 {
    let mut n_n = n;
    while n_n % 10 != n_n {
        n_n = digit_sum(n_n);
    }
    n_n
}
fn digit_sum(n: i64) -> i64 {
    let mut sum = 0;
    let mut n_n = n;
    while n_n != 0 {
        let mod_n = n_n % 10;
        n_n = n_n / 10;
        sum = sum + mod_n;
    }
    sum
}

/// 世界上存在一行代码解决问题的！
/// 已经到神仙的级别了
/// 原理链接https://blog.csdn.net/ray0354315/article/details/53991199
fn nice_job1_digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}
/// 这个是递归写法，只是上面公式的不完全版
fn nice_job2_digital_root(n: i64) -> i64 {
    if n / 10 == 0 {
        n
    } else {
        digital_root(n % 10 + n / 10)
    }
}

#[cfg(test)]
mod date20220131_tests {
    use super::digital_root;
    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }
}
