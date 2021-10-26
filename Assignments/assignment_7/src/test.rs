#[cfg(test)]

pub mod tests {
    use crate::quest1::error_handle::eveodd;

    #[test]
    fn check_even() {
        assert_eq!(eveodd(25), "Enter right integer");
    }
    #[test]
    fn check_odd() {
        assert_eq!(eveodd(4), "even");
    }
    #[test]
    fn check() {
        assert_eq!(eveodd(9), "Enter right integer");
    }
}
