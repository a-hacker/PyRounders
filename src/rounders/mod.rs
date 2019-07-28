pub mod magnitude;

pub fn floor(num: f64, precision: Option<i32>) -> f64{
    let mag = magnitude::get_magnitude(&num);
    let round_precision = precision.unwrap_or(1);

    if mag == 0 || round_precision == 0 {
        0.0
    } else if round_precision > 0 {
        let int_num = num as i64;
        let base: i64 = 10;
        let factor: u32 = (round_precision - 1) as u32;
        let operand: i64 = base.pow(factor);
        let new_num = (int_num / operand * operand) as f64;
        if num < 0.0 && num != new_num {
            new_num - (operand as f64)
        } else {
            new_num
        }
    } else {
        let base: f64 = 10.0;
        let factor: i32 = round_precision.abs();
        let operand: f64 = base.powi(factor);
        let new_num: f64 = ((num * operand) as i64) as f64 / operand;
        if num < 0.0 && num != new_num {
            new_num - (1.0 / operand)
        } else {
            new_num
        }
    }
}

pub fn ceil(num: f64, precision: Option<i32>) -> f64{
    let mag = magnitude::get_magnitude(&num);
    let round_precision = precision.unwrap_or(1);

    if mag == 0 || round_precision == 0 {
        0.0
    } else if round_precision > 0 {
        let int_num = num as i64;
        let base: i64 = 10;
        let factor: u32 = (round_precision - 1) as u32;
        let operand: i64 = base.pow(factor);
        let new_num = (int_num / operand * operand) as f64;
        if num > 0.0 && num != new_num {
            new_num + (operand as f64)
        } else {
            new_num
        }
    } else {
        let base: f64 = 10.0;
        let factor: i32 = round_precision.abs();
        let operand: f64 = base.powi(factor);
        let new_num: f64 = ((num * operand) as i64) as f64 / operand;
        if num > 0.0 && num != new_num {
            new_num + (1.0 / operand)
        } else {
            new_num
        }
    }
}


#[cfg(test)]
mod floor_tests {
    use super::*;

    #[test]
    fn floor_of_zero(){
        assert_eq!(floor(0.0, None), 0.0);
    }

    #[test]
    fn floor_of_zero_with_precision(){
        assert_eq!(floor(0.0, Some(5)), 0.0)
    }

    #[test]
    fn floor_positive_number_default_precision(){
        assert_eq!(floor(15.3, None), 15.0)
    }

    #[test]
    fn floor_positive_number_custom_precision(){
        assert_eq!(floor(15.3, Some(2)), 10.0)
    }

    #[test]
    fn floor_positive_number_no_difference(){
        assert_eq!(floor(16.0, None), 16.0)
    }

    #[test]
    fn floor_negative_number_default_precision(){
        assert_eq!(floor(-15.3, None), -16.0)
    }

    #[test]
    fn floor_negative_number_custom_precision(){
        assert_eq!(floor(-15.3, Some(2)), -20.0)
    }

    #[test]
    fn floor_negative_number_no_difference(){
        assert_eq!(floor(-16.0, None), -16.0)
    }

    #[test]
    fn floor_positive_decimal_default_precision(){
        assert_eq!(floor(0.3, None), 0.0)
    }

    #[test]
    fn floor_positive_decimal_negative_precision(){
        assert_eq!(floor(0.35, Some(-1)), 0.3)
    }

    #[test]
    fn floor_negative_decimal_default_precision(){
        assert_eq!(floor(-0.35, None), -1.0)
    }

    #[test]
    fn floor_negative_decimal_custom_precision(){
        assert_eq!(floor(-0.35, Some(-1)), -0.4)
    }
}

#[cfg(test)]
mod ceil_tests {
    use super::*;

    #[test]
    fn ceil_of_zero(){
        assert_eq!(ceil(0.0, None), 0.0);
    }

    #[test]
    fn ceil_of_zero_with_precision(){
        assert_eq!(ceil(0.0, Some(5)), 0.0)
    }

    #[test]
    fn ceil_positive_number_default_precision(){
        assert_eq!(ceil(15.3, None), 16.0)
    }

    #[test]
    fn ceil_positive_number_custom_precision(){
        assert_eq!(ceil(15.3, Some(2)), 20.0)
    }

    #[test]
    fn ceil_positive_number_no_difference(){
        assert_eq!(ceil(16.0, None), 16.0)
    }

    #[test]
    fn ceil_negative_number_default_precision(){
        assert_eq!(ceil(-15.3, None), -15.0)
    }

    #[test]
    fn ceil_negative_number_custom_precision(){
        assert_eq!(ceil(-15.3, Some(2)), -10.0)
    }

    #[test]
    fn ceil_negative_number_no_difference(){
        assert_eq!(ceil(-16.0, None), -16.0)
    }

    #[test]
    fn ceil_positive_decimal_default_precision(){
        assert_eq!(ceil(0.3, None), 1.0)
    }

    #[test]
    fn ceil_positive_decimal_negative_precision(){
        assert_eq!(ceil(0.35, Some(-1)), 0.4)
    }

    #[test]
    fn ceil_negative_decimal_default_precision(){
        assert_eq!(ceil(-0.35, None), 0.0)
    }

    #[test]
    fn ceil_negative_decimal_custom_precision(){
        assert_eq!(ceil(-0.35, Some(-1)), -0.3)
    }
}