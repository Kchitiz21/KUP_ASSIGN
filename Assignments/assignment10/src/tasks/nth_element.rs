use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};

/// nth function finds the nth element.
///
/// #Arguments
///
/// iterator: It is a List enum.
/// 
/// counter: It is an i32 type variable used to iterate and change in every iteration
///
/// position: It is an i32 type variable stores the value of position
///
/// #Return
///
/// Return the i32 containing the nth element
pub fn nth(position: i32, iterator:List, counter:i32) -> i32 {
    match iterator {
        Nil => -1,
        Cons(current, _re) if counter == position => current,
        Cons(_er, iterator) => nth(position, *iterator, counter + 1),
    }
    
}
