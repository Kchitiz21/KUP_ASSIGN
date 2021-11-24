mod tests {
    pub use futures::executor::block_on;

    pub use crate::tasks::async_2::relate;
    pub use crate::tasks::table_async::check;

    #[test]
    fn table_async_done() {
        assert_eq!(block_on(check()), ());
    }

    #[test]
    fn async_check() {
        assert_eq!(block_on(relate()), ());
    }
}
