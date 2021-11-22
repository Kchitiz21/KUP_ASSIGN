use log::*;
use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};

/// first_repeat function search the first repeated element
///
/// #Arguments
///
/// iterable: It is an list enum.
///
/// random: It is an i32 variable.
///
/// #Return
///
/// Return the i32 containing the third odd number
pub fn first_repeat(iterable:List, random: i32) ->i32 {
      match iterable {
        Nil => -1,

        Cons(initial, _iterable) if initial == random => initial,

        Cons(initial, iterable) => first_repeat(*iterable, initial),
    }
}



