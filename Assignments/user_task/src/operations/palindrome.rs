/// Function
///
/// palindrome: palindrome function checks the given numbers is palindrome or not
///
/// #Arguments
///
///num: num is an i32 type
///
/// #Return
///
/// Returns boolean type
pub fn palindrome(num: i32) -> bool {
    let mut result: i32 = num;
    let mut remain: i32;
    let mut pallin: i32 = 0;
    while result != 0 {
        remain = result % 10;
        pallin = pallin * 10 + remain;
        result /= 10;
    }
    log::info!("Given number is pallindrome");
    pallin == num
}
