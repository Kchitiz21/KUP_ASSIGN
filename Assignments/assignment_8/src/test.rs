#[cfg(test)]
/// Compare Generic Numbers
pub use crate::quest1::generic_min::minimum;
#[test]
pub fn generic() {
    assert_eq!(minimum(10.7, 10.4),10.4);
}
#[test]
pub fn compare() {
    assert_eq!(minimum(1, 100), 1);
}

/// Sorting Generic Numbers
pub use crate::quest1::generic_sort::sort;
#[test]
pub fn sorting_done() {
    assert_eq!(
        sort(&mut vec![5, 1, 7, 8, 2, 9]),
        vec![1, 2, 5, 7, 8, 9]
    );
}

pub use crate::iterator::GeometricSeries;
pub  use crate::iterator::Iterator;

#[test]
pub fn gp_run() {
    let values = GeometricSeries {
        first_number: 2,
        ratio: 2,
        length: 11,
    };
    assert_eq!(
        GeometricSeries::generate(values.first_number, values.ratio, values.length),
        [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
    );
}
#[test]
pub fn gp_check() {
    let values = GeometricSeries {
        first_number: 1,
        ratio: 2,
        length: 11,
    };
    assert_eq!(
        GeometricSeries::generate(values.first_number, values.ratio, values.length),
        [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
}