/// Function 'minimum' compares two given numbers to find the smaller number
///
/// #Arguments
///
/// Taking T as input from 'test.rs'
///
/// #Return
///
/// Returns  T as a minimum number
pub fn minimum<T: PartialOrd>(first_number: T, second_number: T) -> T {
    if first_number < second_number {
        return first_number;
    }
    second_number
}