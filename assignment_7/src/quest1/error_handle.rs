/// enter_eveodd functions checks the number is even or odd
///
/// #Arguments
///
/// result : result control the number-value
///
/// #Return
///
/// return : return handle error and give output String.
pub fn enter_eveodd(num1: i32) -> Result<String, String> {
    if num1 % 2 == 0 {
        Ok("even".to_string())
    } else {
        Err("odd".to_string())
    }
}
/// eveodd which handle the response of calling function
///
/// #Arguments
///
///num1: num1 is an Integer use to operate
///
/// #Return
///
/// Returns String which gives the function output
pub fn eveodd(num1: i32) -> String {
    let answer = enter_eveodd(num1);
    match answer {
        Ok(answer) => answer,
        Err(_v) => "Enter right integer".to_string(),
    }
}
