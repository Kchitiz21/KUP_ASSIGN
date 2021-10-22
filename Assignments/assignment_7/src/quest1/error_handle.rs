/// enter_eveodd function handles the calling function 
///
/// #Arguments
///
///num1:num1 is Integer which use to check even or odd
///
/// #Return
///
/// Returns Result which used to handle Error
pub fn enter_eveodd(num1: i32) -> Result<String, String> {
    if num1 % 2 == 0 {
        Ok("even".to_string())
    } else {
        Err("odd".to_string())
    }
}
/// eveodd function handles calling function
///
/// #Arguments
///
///num1:num1 is Integer which use to operate
///
/// #Return
///
/// Returns String which operates the output
pub fn eveodd(num1: i32) -> String {
    let answer = enter_eveodd(num1);
    match answer {
        Ok(answer) => answer,
        Err(_v) => "Enter right integer".to_string(),
    }
}
