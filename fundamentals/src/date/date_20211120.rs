pub fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops
        .iter()
        .fold(0i32, |acc, &(on, off)| acc + on - off)
    // bus_stops.iter().map(|(a,b)| a-b).sum()
}



/// # Who likes it?
///
/// []                                -->  "no one likes this"
/// ["Peter"]                         -->  "Peter likes this"
/// ["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
/// ["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
/// ["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"
///
pub fn likes(names: &[&str]) -> String {
    // let length = names.len();
    // match length {
    //     0 => String::from("no one likes this"),
    //     1 => format!("{} likes this",names[0]),
    //     2 => format!("{} and {} like this",names[0],names[1]),
    //     3 => format!("{}, {} and {} like this",names[0],names[1],names[2]),
    //     _ => format!("{}, {} and {} others like this",names[0],names[1],length-2)
    // }
    match names {
        [] => String::from("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}

/// #Playing with digits
///89 --> 8¹ + 9² = 89 * 1
/// 695 --> 6² + 9³ + 5⁴= 1390 = 695 * 2
/// 46288 --> 4³ + 6⁴+ 2⁵ + 8⁶ + 8⁷ = 2360688 = 46288 * 51
/// For number "abcd" Is there an integer k such as : (a ^ p + b ^ (p+1) + c ^(p+2) + d ^ (p+3) + ...) = n * k
pub fn dig_pow(n: i64, p: i32) -> i64 {
    let mut digit_list = vec!();
    let mut tmp = n;
    while tmp > 0 {
        digit_list.push(tmp % 10);
        tmp /= 10;
    }
    // 翻转字符串符合顺序
    digit_list.reverse();
    let mut i = -1;
    let sum:i64 = digit_list.iter().map(|x| {
        i += 1;
        x.pow((p+i) as u32)
    }).sum();
    println!("sum is {}",sum);
    if sum % n == 0{
        sum / n
    } else {
        -1
    }
}


#[cfg(test)]
mod date20211120_tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
        assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
        assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
      }

      #[test]
      fn example_tests() {
          assert_eq!(likes(&[]), "no one likes this");
          assert_eq!(likes(&["Peter"]), "Peter likes this");
          assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
          assert_eq!(
              likes(&["Max", "John", "Mark"]),
              "Max, John and Mark like this"
          );
          assert_eq!(
              likes(&["Alex", "Jacob", "Mark", "Max"]),
              "Alex, Jacob and 2 others like this"
          );
      }

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
