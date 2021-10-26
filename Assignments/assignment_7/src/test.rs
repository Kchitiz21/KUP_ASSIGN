#[cfg(test)]

mod tests {
    use crate::quest1::error_handle::eveodd;

    #[test]
    fn check_even() {
        assert_eq!(eveodd(25), "Enter right integer".to_string());
    }
      #[test]
    fn check_next() {
        assert_eq!(eveodd(3), "Please put right integer".to_string());
    }
    #[test]
    fn error_check() {
        assert_eq!(eveodd(4), "Enter right integer".to_string());
    }
}
