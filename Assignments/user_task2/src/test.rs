#[cfg(test)]
pub mod tests {
    use crate::operations::{armstrong, automorphic, matrix::matrix, neon, pallindrome};
    #[test]
    fn armstrong_success() {
        assert_eq!(armstrong::armstrong(1634), true);
    }
    #[test]
    fn armstrong_failure() {
        assert_eq!(armstrong::armstrong(1500), false);
    }
    #[test]
    fn automorphic_success() {
        assert_eq!(automorphic::automorphic(5), true);
    }
    #[test]
    fn automorphic_failure() {
        assert_eq!(automorphic::automorphic(7), false);
    }
    #[test]
    fn neon_success() {
        assert_eq!(neon::neon(9), true);
    }
    #[test]
    fn neon_failure() {
        assert_eq!(neon::neon(12), false);
    }
    #[test]
    fn palindrome_success() {
        assert_eq!(pallindrome::palindrome(1221), true);
    }
    #[test]
    fn palindrome_failure() {
        assert_eq!(pallindrome::palindrome(1222), false);
    }
    #[test]
    fn multiplication_first_success() {
        let first = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let second = [[1, 0, 0].to_vec(), [0, 1, 0].to_vec(), [0, 0, 1].to_vec()].to_vec();
        let right = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        assert_eq!(matrix(&first, &second), Ok(right));
    }
    #[test]
    fn multiplication_second_success() {
        let first = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let second = [[1, 0].to_vec(), [0, 1].to_vec()].to_vec();
        let right = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        assert_eq!(matrix(&first, &second), Ok(right));
    }
    #[test]
    fn multiplication_first_failure() {
        let first = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let second = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        assert_eq!(
            matrix(&first, &second),
            Err("Matrix Multiplication is not possible".to_string())
        );
    }
    #[test]
    fn multiplication_second_failure() {
        let first = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let second = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        assert_eq!(
            matrix(&first, &second),
            Err("Matrix Multiplication is not possible".to_string())
        );
    }
}
