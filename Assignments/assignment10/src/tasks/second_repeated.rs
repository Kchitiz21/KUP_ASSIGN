use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};
/// second function finds the second repeated.
///
/// #Arguments
///
/// iterable is an enum object which contains the list of numbers.
///
/// repeat is used to stores count to find the second number
///
///previous An i32 variable containing the previous Cons in Cons tuple of List enum.
///
/// #Return
///
/// Return the i32 number contains the third odd number

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


  
