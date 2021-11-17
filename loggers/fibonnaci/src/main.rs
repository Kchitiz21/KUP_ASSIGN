use log::info;
fn fibonacci (num: u32) -> u32 {
    if num <= 0 {
        return 0;
    } else if num == 1{
        return 1;
    }
    fibonacci(num - 1)  + fibonacci(num - 2)
}
fn main( ){
    env_logger::init();
    for value in 1..8{
        info!("fibonacci ({}) = {}", value, fibonacci(value));
    }
}
