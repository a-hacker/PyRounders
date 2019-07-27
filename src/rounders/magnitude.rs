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