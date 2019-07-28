pub mod magnitude;

pub fn floor(num: f64, precision: Option<i32>) -> f64{
    let round_precision = precision.unwrap_or(0);
    let base: f64 = 10.0;

    let temp_num = num * base.powi(-1 * round_precision);
    temp_num.floor() * base.powi(1 * round_precision)
}

pub fn ceil(num: f64, precision: Option<i32>) -> f64{
    let round_precision = precision.unwrap_or(0);
    let base: f64 = 10.0;

    let temp_num = num * base.powi(-1 * round_precision);
    temp_num.ceil() * base.powi(1 * round_precision)
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
        assert_eq!(floor(15.3, Some(1)), 10.0)
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
        assert_eq!(floor(-15.3, Some(1)), -20.0)
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
        assert_eq!(ceil(15.3, Some(1)), 20.0)
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
        assert_eq!(ceil(-15.3, Some(1)), -10.0)
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