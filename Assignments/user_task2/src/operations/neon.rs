/// Function
///
/// neon: neon function checks the given numbers is neon number or not
///
/// #Arguments
///
///num: num is an i32 type
///
/// #Return
///
/// Returns boolean type

pub fn neon(num: i32) -> bool {
    let mut result = num * num;
    let mut sum: i32 = 0;
    while result > 0 {
        sum += result % 10;
        result /= 10;
    }
    log::info!("Given number is neon number");
    num == sum
}
