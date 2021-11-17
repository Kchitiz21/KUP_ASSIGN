use std::collections::HashMap;
/// sum is the function which return the sum of ages
///
/// #Arguments
///
/// map - A map is Hashmap object
///
///
/// #Return
///
/// Return Result<i32,String> enum
pub fn sum(map: HashMap<String, i32>, string: String) -> Result<i32, String> {
    if map.is_empty() {
        return Err("Given hashmap is empty".to_string());
    }
    let mut total = 0;
    let length = string.len();
    for (key, value) in map.iter() {
        for index in 0..key.len() - length + 1 {
            if key[index..index + length] == string {
                total += value;
            }
        }
    }
    Ok(total)
}
