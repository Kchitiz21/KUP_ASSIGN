#[cfg(test)]
pub mod tests {
    use crate ::response::root::json;


    #[test]
    fn body_success() {
        env_logger::init();
        let value = "name".to_string();
        assert_eq!(json(value).unwrap(),"ditto".to_string())

    }
    #[test]
    fn body_fail() {
        let value = "nae".to_string();
        assert_eq!(json(value).unwrap(),"Error".to_string())
    }
}

