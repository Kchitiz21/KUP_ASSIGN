#[cfg(test)]
mod test {
    use crate::tasks::book_info::Library;
    #[test]
    fn add_check() {
        let mut library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let input = library.add_book("poe".to_string(), "Jameson".to_string(), 6, 1);
        assert_eq!(input, ())
    }

    #[test]
    fn book_display_success() {
        let mut library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.book_display();
        assert_eq!(output, Ok("Data showed".to_string()))
    }

    #[test]
    fn book_display_failure() {
        let mut library = Library {
            title: vec![],
            author: vec![],
            access_num: vec![],
            flag: 0,
        };
        let output = library.book_display();
        assert_eq!(output, Err("Empty".to_string()))
    }

    #[test]
    fn count_book() {
        let book_check_1 = Library {
            title: vec!["The light house".to_string()],
            author: vec!["Virginia Woolf".to_string()],
            access_num: vec![01],
            flag: 1,
        };

        assert_eq!(book_check_1.total_book(), Ok(1));
    }

    #[test]
    fn count_book_fail() {
        let book_check_1 = Library {
            access_num: vec![],
            author: vec![],
            title: vec![],
            flag: 1,
        };

        assert_eq!(book_check_1.total_book(), Err(0));
    }

    #[test]
    fn title_done() {
        let library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.title_name("Chernobyl".to_string());
        assert_eq!(output,Ok("The book is present withe given title".to_string())
        )
    }

    #[test]
    fn title_fail() {
        let library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.title_name("Chenobyl".to_string());
        assert_eq!(output, Err("Hey book is not present".to_string()));
    }

    #[test]
    fn author_name_done() {
        let library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.author_name("Edgar".to_string());
        assert_eq!(output,Ok("The book is present with given author".to_string())
        )
    }

    #[test]
    fn author_name_fail() {
        let library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.author_name("hoffman".to_string());
        assert_eq!(output, Err("Hey book is not present".to_string()));
    }

    #[test]
    fn issue_book_done() {
        let mut library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.issue_library("Chernobyl".to_string());
        assert_eq!(output, Ok("Book issued".to_string()))
    }
    #[test]
    fn issue_book_fail() {
        let mut library = Library {
            title: vec![
                "Chernobyl".to_string(),
                "Money heist".to_string(),
            ],
            author: vec!["howard".to_string(), "Edgar".to_string()],
            access_num: vec![1, 2],
            flag: 0,
        };
        let output = library.issue_library("Chrnobyl".to_string());
        assert_eq!(output, Err("Hey this is not present".to_string()))
    }
}