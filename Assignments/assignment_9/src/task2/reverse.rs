/// reverse is a function which reverse the list
///
/// #Arguments
///
/// list - A list is Vector object which contains integers
///
/// #Return
///
/// Return Result<Vec<i32>,String> enum
pub fn reverse(mut list: Vec<i32>) -> Result<Vec<i32>, String> {
    if list.is_empty() {
        return Err("Given list is empty".to_string());
    }
    let mut start_index = 0;
    let mut end_index = list.len() - 1;
    let mut temp;
    while start_index < end_index {
        temp = list[start_index];
        list[start_index] = list[end_index];
        list[end_index] = temp;
        start_index += 1;
        end_index -= 1;
    }
    Ok(list)
}
