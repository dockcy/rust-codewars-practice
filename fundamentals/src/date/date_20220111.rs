
fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    fn factor_sq_sum(n: u64) -> u64 {
        let sqrt = (n as f64).sqrt() as u64;
        (1..=sqrt).filter_map(|i| {
            match n % i == 0 {
                true => match n / i == sqrt {
                    true => Some(n),
                    false => Some(i*i + (n / i).pow(2))
                },
                _ => None
            }
        }).sum()
    }
    fn is_square(n: u64) -> bool {
        ((n as f64).sqrt() as u64).pow(2) == n
    }

    (m..n)
        .filter_map(|i| {
            let sum = factor_sq_sum(i);
            match is_square(sum) {
                true => Some((i, sum)),
                false => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod date20220111_tests{
    use super::*;
    fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(list_squared(m, n), exp)
    }
    
    #[test]
    fn basics_list_squared() {
    
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(42, 250, vec![(42, 2500), (246, 84100)]);
        testing(300, 600, vec![]);
        
    }
}