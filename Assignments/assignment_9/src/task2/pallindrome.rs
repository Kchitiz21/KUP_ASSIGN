/// palindrome is the function which is used to check whether given list is a palindrome or not
///
/// #Arguments
///
/// list - A list is Vector object which contains numbers
///
/// #Return
///
/// Return Result<bool,String> enum
pub fn palindrome(vect: Vec<i32>) -> bool {
    let mut vect_2 = Vec::new();
    let length = vect.len() - 1;
    for loop1 in 0..=length {
        if vect[loop1] == vect[length - loop1] {
            vect_2.push(vect[loop1]);
        }
    }
    vect == vect_2
}
