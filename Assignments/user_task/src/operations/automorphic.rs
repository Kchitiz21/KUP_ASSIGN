/// Function
///
/// automorphic: automorphic function checks the given numbers is automorphic or not
///
/// #Arguments
///
///num: num is an i32 type
///
/// #Return
///
/// Returns boolean type

pub fn automorphic(mut num: i32) -> bool {
    let mut result = num * num;
    while num > 0 {
        if num % 10 != result % 10 {
            return false;
        }
        num /= 10;
        result /= 10;
    }
    log::info!("This number is automorphic");
    true
}
