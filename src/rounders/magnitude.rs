pub fn get_magnitude(num: &f64) -> i32 {
    if *num == 0.0 {
        0
    }
    else if *num < 1.0 && *num > -1.0 {
        get_negative_magnitude(num)
    } else {
        get_positive_magnitude(num)
    }
}

fn get_positive_magnitude(num: &f64) -> i32 {
    let mut i = 1;
    let mut j = 0;
    let val = *num as i64;
    while val / i != 0 {
        i *= 10;
        j += 1;
    }
    j
}

fn get_negative_magnitude(num: &f64) -> i32 {
    let mut val = *num;
    let mut magnitude = 0;
    while val < 1.0 && val > -1.0 {
        val *= 10.0;
        magnitude -= 1;
    }
    magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_magnitude_of_zero(){
        let test_val: f64 = 0.0;
        assert_eq!(get_magnitude(&test_val), 0)
    }

    #[test]
    fn get_magnitude_of_positive_single_digit_number(){
        let test_val: f64 = 4.0;
        assert_eq!(get_magnitude(&test_val), 1)
    }

    #[test]
    fn get_magnitude_of_negative_single_digit_number(){
        let test_val: f64 = -4.0;
        assert_eq!(get_magnitude(&test_val), 1)
    }

    #[test]
    fn get_magnitude_of_positive_multi_digit_number(){
        let test_val: f64 = 104.0;
        assert_eq!(get_magnitude(&test_val), 3)
    }

    #[test]
    fn get_magnitude_of_negative_multi_digit_number(){
        let test_val: f64 = -104.0;
        assert_eq!(get_magnitude(&test_val), 3)
    }

    #[test]
    fn get_magnitude_of_positive_decimal_number(){
        let test_val: f64 = 0.01;
        assert_eq!(get_magnitude(&test_val), -2)
    }

    #[test]
    fn get_magnitude_of_negative_decimal_number(){
        let test_val: f64 = -0.01;
        assert_eq!(get_magnitude(&test_val), -2)
    }
}