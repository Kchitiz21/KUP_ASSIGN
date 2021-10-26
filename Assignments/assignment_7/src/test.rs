#[cfg(test)]

mod tests {
    use crate::quest1::error_handle::eveodd;

    #[test]
    fn check_even() {
        assert_eq!(eveodd(25), "Enter right integer".to_string());
    }
    #[test]
    fn error_check() {
        assert_ne!(eveodd(4), "Enter right integer".to_string());
    }
}
