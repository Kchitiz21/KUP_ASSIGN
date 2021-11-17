/// duplicate is a function which is used to make duplicates
///
/// #Arguments
///
/// list - A list is Vector object which contains integers
///
/// #Return
///
/// Return Result<Vec<i32>,String> enum
pub fn duplicate(list: Vec<i32>) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut vec: Vec<i32> = Vec::new();
    for element in list {
        vec.push(element);
        vec.push(element);
    }
    Ok(vec)
}
