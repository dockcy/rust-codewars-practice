/// Title : Common denominators
/// Use Least Common Multiple to expand of many fractions to a common denominator
/// input: (numerator1,denominator1)...
fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let d_list:Vec<i64> = l.iter().map(|(n,d)| *d).collect();
    let lcm = d_list.into_iter().reduce(|acc,next| {
        let tmp = acc * next / gcd(acc, next);
        println!("lcmming => {}",tmp);
        tmp
    }).unwrap();
    l.into_iter().map(|(n,d)| (n * (lcm / d),lcm)).collect()
}

fn gcd(m: i64, n: i64) -> i64 {
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


#[cfg(test)]
mod date20220116_tests{
    use super::*;
    fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(convert_fracts(l), exp)
    }
    
    #[test]
    fn basics_convert_fracts() {
        
        testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
        testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    
    }
}