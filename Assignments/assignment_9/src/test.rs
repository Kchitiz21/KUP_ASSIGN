#[cfg(test)]
mod tests {
    use crate::task1::hash::sum;
    use crate::task2::compress::compress;
    use crate::task2::drop::remove_nth;
    use crate::task2::duplicate::duplicate;
    use crate::task2::even::even;
    use crate::task2::pallindrome::palindrome;
    use crate::task2::reverse::reverse;

    #[test]
    fn hash_okay() {
        use std::collections::HashMap;
        let mut first_map = HashMap::new();
        first_map.insert("parmanu".to_string(), 5);
        first_map.insert("anand".to_string(), 13);
        first_map.insert("anant".to_string(), 14);
        first_map.insert("aman".to_string(), 22);
        first_map.insert("maann".to_string(), 25);
        assert_eq!(sum(first_map, "ana".to_string()), Ok(27));
    }

    #[test]
    fn hash_empty() {
        use std::collections::HashMap;
        let third = HashMap::new();
        assert_eq!(
            sum(third, "uy".to_string()),
            Err("Given hashmap is empty".to_string())
        );
    }

    #[test]
    pub fn check_palindrome_success() {
        assert_eq!(palindrome(vec![1, 2, 3, 2, 1]), true);
    }

    #[test]
    pub fn check_palindrome() {
        assert_eq!(palindrome(vec![1, 2, 3, 2, 9]), false);
    }

    #[test]
    fn reverse_okay() {
        assert_eq!(reverse(vec![6, 8, 9]), Ok(vec![9, 8, 6]));
    }

    #[test]
    fn reverse_empty() {
        assert_eq!(reverse(vec![]), Err("Given list is empty".to_string()));
    }

    #[test]
    fn even_okay() {
        let list = vec![55, 59, 80];
        assert_eq!(even(list), Ok(80));
    }

    #[test]
    fn even_empty() {
        let list = vec![];
        assert_eq!(even(list), Err("Given list is empty".to_string()));
    }

    #[test]
    fn duplicate_okay() {
        let list = vec![0, 0, 1, 1];
        assert_eq!(compress(list), Ok(vec![0, 1]));
    }

    #[test]
    fn duplicate_empty() {
        let list = vec![];
        assert_eq!(compress(list), Err("Given list is empty".to_string()));
    }

    #[test]
    fn duplicate_done() {
        let list = vec![1, 2];
        assert_eq!(duplicate(list), Ok(vec![1, 1, 2, 2]));
    }

    #[test]
    fn duplicate_clear() {
        let list = vec![];
        assert_eq!(duplicate(list), Err("Given list is empty".to_string()));
    }

    #[test]
    fn remove_nth_value() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nth_value = 3;
        assert_eq!(
            remove_nth(list, nth_value),
            Ok(vec![1, 2, 4, 5, 6, 7, 8, 9])
        );
    }

    #[test]
    fn remove_empty() {
        let list = vec![];
        let nth_value = 10;
        assert_eq!(
            remove_nth(list, nth_value),
            Err("Given list is empty".to_string())
        );
    }
}
