use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};
/// second function finds the second repeated element.
///
/// #Arguments
///
/// iterable: It is an List enum.
///
/// repeat: It is an i32 used to stores count to find the second number
///
/// previous: It is i32 variable.
///
/// #Return
///
/// Return i32 containing the third odd number

pub fn second(previous:i32, iterable:List, repeat: i32) ->i32 {
    match iterable {
        Nil => -1,
        Cons(initial, _) if initial == previous && repeat == 1 => initial,
        Cons(initial, iterable) if initial == previous => {
            second(initial, *iterable, repeat + 1)
        }
        Cons(initial, iterable) => second(initial, *iterable, repeat),
    }
   
}


  
