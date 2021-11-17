use log::info;
fn main() {
env_logger::init();
    let string: &str = "MAM";
    let length = string.len();
    for loop_1 in 0..length {
        if &string.as_bytes()[loop_1] != &string.as_bytes()[length - loop_1 - 1] {
            info!("This is not an example of Palindrome");
            break;
        }
        else {
            info!(" This is an example of Palindrome");
            break;
        }
    }
}
