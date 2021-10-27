#[cfg(test)]

pub use crate::quest1::generic_min::minimum;
#[test]
pub fn generic() {
    assert_eq!(minimum(17.4, 14.4),14.4);
}
#[test]
pub fn compare() {
    assert_eq!(minimum(2,100), 2);
}

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
        first_number: 3,
        ratio: 3,
        length: 8,
    };
    assert_eq!(
        GeometricSeries::generate(values.first_number, values.ratio, values.length),
        [3, 9, 27, 81, 243, 729, 2187, 6561]
    );
}
#[test]
pub fn gp_check() {
    let values = GeometricSeries {
        first_number: 4,
        ratio: 4,
        length: 8,
    };
    assert_eq!(
        GeometricSeries::generate(values.first_number, values.ratio, values.length),
        [4, 16, 64,256, 1024, 4096, 16384, 65536]
    );
}
