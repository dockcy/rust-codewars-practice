/// The Fibonacci numbers are the numbers in the following integer sequence (Fn):
///
/// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, ...
///
/// such as
///
/// F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.
///
/// Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying
///
/// F(n) * F(n+1) = prod.
///
/// Your function productFib takes an integer (prod) and returns an array:
///
/// [F(n), F(n+1), true] or {F(n), F(n+1), 1} or (F(n), F(n+1), True)
/// depending on the language if F(n) * F(n+1) = prod.
///
/// If you don't find two consecutive F(n) verifying F(n) * F(n+1) = prodyou will return
///
/// [F(n), F(n+1), false] or {F(n), F(n+1), 0} or (F(n), F(n+1), False)
/// F(n) being the smallest one such as F(n) * F(n+1) > prod.

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut f_n_2 = 0;
    let mut f_n_1 = 1;
    if prod == 0 {
        return (0, 1, true);
    }
    if prod == 1 {
        return (1, 1, true);
    }
    while true {
        if f_n_1 * f_n_2 > prod {
            return (f_n_2, f_n_1, false);
        }
        if f_n_1 * f_n_2 == prod {
            return (f_n_2, f_n_1, true);
        }
        let mut tmp = f_n_1 + f_n_2;
        f_n_2 = f_n_1;
        f_n_1 = tmp;
    }
    return (0, 0, true);
}

#[cfg(test)]
mod date20211213_tests {
    use super::*;
    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
