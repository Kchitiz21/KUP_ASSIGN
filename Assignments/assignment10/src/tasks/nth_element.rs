use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};

/// nth function finds the nth element.
///
/// #Arguments
///
/// iterable is an enum object.
/// 
/// counter used to iterate and change in every iteration
///
/// position stores the value of position
///
/// #Return
///
///  Return the i32 number contains the third odd number
pub fn nth(position: i32, iterator:List, counter:i32) -> i32 {
    match iterator {
        Nil => -1,
        Cons(current, _re) if counter == position => current,
        Cons(_er, iterator) => nth(position, *iterator, counter + 1),
    }
    
}
