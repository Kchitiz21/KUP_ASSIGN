pub mod test;

use log::*;
pub use std::io;
use text_io::read;
pub mod operations {
    pub mod armstrong;
    pub mod automorphic;
    pub mod matrix;
    pub mod neon;
    pub mod pallindrome;
}
pub use crate::operations::armstrong::armstrong;
pub use crate::operations::automorphic::automorphic;
pub use crate::operations::matrix::matrix;
pub use crate::operations::neon::neon;
pub use crate::operations::pallindrome::palindrome;

fn main() {
    env_logger::init();
    loop {
        info!(
            "------Operations------ \n
    1:- Check the given number is Armstrong number or not\n
    2:- Check the given number is Automorphic number or not\n
    3:- Check the given number is Neon number or not\n
    4:- Check the given number is Palindrome number or not\n
    5:- Perform matrix multiplication\n
    (Press -1 to exit)\n"
        );
        let choice: i32 = read!();
        if choice == 1 {
            info!("Enter a number",);
            let number: i32 = read!();
            debug!("{}", armstrong(number));
        }
        if choice == 2 {
            info!("Enter a number");
            let number: i32 = read!();
            debug!("{}", automorphic(number));
        }
        if choice == 3 {
            info!("Enter a number");
            let number: i32 = read!();
            debug!("{}", neon(number));
        }
        if choice == 4 {
            info!("Enter a number");
            let number: i32 = read!();
            debug!("{}", palindrome(number));
        }
        if choice == 5 {
            info!("Enter the number of rows for first matrix\n");
            let row_1: i32 = read!();
            info!("Enter the number of columns for first matrix\n");
            let column_1: i32 = read!();
            let mut first_matrix: Vec<Vec<i32>> = Vec::new();
            info!("Enter the entries rowwise");
            for _index_1 in 0..row_1 {
                let mut vec: Vec<i32> = Vec::new();
                for _index_2 in 0..column_1 {
                    vec.push(read!())
                }
                first_matrix.push(vec);
            }
            info!("Enter the number of rows for second matrix\n");
            let row_2: i32 = read!();
            info!("Enter the number of columns for second matrix\n");
            let column_2: i32 = read!();
            let mut second_matrix: Vec<Vec<i32>> = Vec::new();
            info!("Enter the entries rowwise");
            for _index_1 in 0..row_2 {
                let mut vec: Vec<i32> = Vec::new();
                for _index_2 in 0..column_2 {
                    vec.push(read!())
                }
                second_matrix.push(vec);
            }
            debug!("{:?}", matrix(&first_matrix, &second_matrix));
        }
        if choice == -1 {
            break;
        }
    }
}
