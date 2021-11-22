mod tests {
   pub use crate::tasks::nth_element::nth;
    pub use crate::tasks::first_repeated::first_repeat;
    pub use crate::tasks::second_repeated::second;
    pub use crate::tasks::third_odd::find_odd;
    pub use crate::datastore::List::{Nil, Cons};

    #[test]
    fn nth_search() {
        env_logger::builder().is_test(true).try_init();
        let test_list = Cons(
            21,
            Box::new(Cons(
                22,
                Box::new(Cons(
                    33,
                    Box::new(Cons(
                        34,
                        Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(nth(4, test_list,1), 34);
        log::info!("This is nth element element result");

    }

    #[test]
    fn nth_search_new() {
         env_logger::builder().is_test(true).try_init();
       let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(nth(3, test_list,1), 3);
        log::info!("This is nth element element result");

    }

    #[test]
    fn first_repeated() {
        env_logger::builder().is_test(true).try_init();
        let list_val = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(
                        43,
                        Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_repeat(list_val,-1 ),21 );
        log::info!("This is first repeating element result")


    }

    #[test]
    fn first_repeated_nrw() {
         env_logger::builder().is_test(true).try_init();
         let list_val = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    12,
                    Box::new(Cons(
                        43,
                        Box::new(Cons(5, Box::new(Cons(7, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_repeat(list_val,-1),-1);
        log::info!("This is first repeating element result");
    }
    #[test]
    fn second_repeat_() {
         env_logger::builder().is_test(true).try_init();
        let box_array = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(7, Box::new(Cons(9, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(second(21, box_array, 2),-1);
        log::info!("This is second repeating element result");
    }
    #[test]
    fn second_repeat_new() {
         env_logger::builder().is_test(true).try_init();
        let box_array = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    32,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(7, Box::new(Cons(9, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(second(21, box_array, 2),-1);
        log::info!("This is second repeating element result");
    }

    #[test]
    fn third_odd() {
        env_logger::builder().is_test(true).try_init();
        let test_data = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    35,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(find_odd(test_data,3), 35);
        log::info!("This is the third odd element result");
    }
    #[test]
    fn third_odd_new() {
        env_logger::builder().is_test(true).try_init();
        let test_data = Cons(
            21,
            Box::new(Cons(
                24,
                Box::new(Cons(
                    38,
                    Box::new(Cons(
                        45,
                        Box::new(Cons(96, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(find_odd(test_data,3), -1);
        log::info!("This is the third odd element result");
    }
}


