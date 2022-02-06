use std::ascii::AsciiExt;

/// find the smallest item in an array
fn find_smallest_int(arr:&[i32]) -> i32{
    match arr.iter().min() {
        Some(&i) => i,
        None => 0
    }
}

/// Title:Regex validate PIN code
/// ATM machines allow 4 or 6 digit PIN codes and PIN codes cannot contain anything but exactly 4 digits or exactly 6 digits.
/// If the function is passed a valid PIN string, return true, else return false.
/// Examples (Input --> Output)
/// "1234"   -->  true
/// "12345"  -->  false
/// "a234"   -->  false
fn validate_pin(pin: &str) -> bool {
    fn get_digit_nums(s:&str) -> i32 {
        s.chars().fold(0,|mut digit_nums,c| { if c >= '0' && c <= '9' { digit_nums += 1; digit_nums} else { digit_nums } })
    }
    match pin.len() {
        4 => match  get_digit_nums(pin) {
            4 => true,
            _ => false
        }
        6 => match get_digit_nums(pin) {
            6 => true,
            _ => false
        }
        _ => false
    }
}

fn nice_job_validate_pin(pin:&str) -> bool {
    (pin.len() == 4 || pin.len() == 6) && pin.chars().all(|c| c.is_ascii_digit())
}
#[cfg(test)]
mod date20220128_tests {
    use super::validate_pin;
    
    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }
    
    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }
    
    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
}