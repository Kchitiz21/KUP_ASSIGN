use log::*;
use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};

/// first_repeat function search the first repeated element
///
/// #Arguments
///
/// iterable is an enum object which contains the list of numbers.
///
/// random is an i32 variable containing the previous Cons in Cons tuple of List enum.
///
/// #Return
///
///  Return the i32 number contains the third odd number
pub fn first_repeat(iterable:List, random: i32) ->i32 {
    info!("This is the code for first repeated number");
    match iterable {
        Nil => -1,

        Cons(initial, _iterable) if initial == random => initial,

        Cons(initial, iterable) => first_repeat(*iterable, initial),
    }
}



