use crate::datastore::List;
use crate::datastore::List::{Nil, Cons};
/// find_odd function finds the third odd number.
///
/// #Arguments
///
/// iterable: It is an List enum.
///
/// iterator: It is an i32 variable.
///
/// #Return
///
/// Return i32 containing the third odd number

pub fn find_odd(iterable:List,iterator:i32) ->i32 {
    match iterable {
        Nil => -1,
        Cons(initial, _iterable) if iterator == 1 && &initial & 1 == 1 => initial,

        Cons(initial, iterable) if &initial & 1 == 1 => find_odd(*iterable, iterator - 1),

        Cons(_initial, iterable) => find_odd(*iterable, iterator),
    }
}
