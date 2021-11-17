use log::info;
fn main() {
    env_logger::init();
    let years_arr: [i32; 11] = [2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010];
    let mut count = 0;
    for i in 0..11 {
        if years_arr[i] % 4 == 0 {
            if years_arr[i] % 100 == 0 {
                if years_arr[i] % 400 == 0 {
                    info!("{:?} is leap year", years_arr[i]);
                    count = count + 1;
                } else {
                    info!("{:?} is not leap year", years_arr[i]);
                }
            } else {
                info!("{:?} is leap year", years_arr[i]);
                count = count + 1;
            }
        } else {
            info!("{:?} is not leap year", years_arr[i]);
        }
    }
    info!("Count of Leap Years is: {}", count);
}
