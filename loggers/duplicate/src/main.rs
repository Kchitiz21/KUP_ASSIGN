use log::info;
fn main() {
env_logger::init();
    let string: &str = "Kchitiz Deep";
    let length = string.len();
    let mut count: i32;
    let mut char_1: Vec<char> = string.chars().collect();
    for loop_1 in 0..length {
        count = 1;
        for loop_2 in loop_1 + 1..length {
            if char_1[loop_1] == char_1[loop_2] && char_1[loop_1] != ' ' {
                count = count + 1;
                char_1[loop_2] = '0';
            }
        }
        if count > 1 && char_1[loop_1] != '0' {
           info!("{}", char_1[loop_1]);
        }
    }
}
