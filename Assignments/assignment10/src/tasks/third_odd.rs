use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};
/// find_odd function finds the third odd number.
///
/// #Arguments
///
/// iterable is an enum object which contains the list of numbers.
///
/// iterator used to find the odd number.
///
/// #Return
///
/// Return the i32 number contains the third odd number

pub fn find_odd(iterable:List,iterator:i32) ->i32 {
    match iterable {
        Nil => -1,
        Cons(initial, _iterable) if iterator == 1 && &initial & 1 == 1 => initial,

        Cons(initial, iterable) if &initial & 1 == 1 => find_odd(*iterable, iterator - 1),

        Cons(_initial, iterable) => find_odd(*iterable, iterator),
    }
}
