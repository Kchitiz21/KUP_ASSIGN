use log::info;
///enum 'IpAddress' which have variants for classes of ipaddress
enum _IpAddress {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}

/// function 'check_ip' which is used check the given ip_Address
///
/// #Arguments
///
/// ip: is tuple object of unsigned integer type
///
/// #Return
///
/// Returns Result enum which used give the Class Of Ip
fn check_ip(ip: (u128, u128, u128, u128)) {
    match ip {
        (1..=126, 0..=255, 0..=255, 1..=254) => {
            info!("IpAddress::ClassA({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (128..=191, 0..=255, 0..=255, 1..=254) => {
            info!("IpAddress::ClassB({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (192..=223, 0..=255, 1..=254, 1..=254) => {
            info!("IpAddress::ClassC({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (224..=239, 0..=255, 0..=255, 0..=255) => {
            info!("IpAddress::ClassD({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        (240..=254, 0..=255, 0..=255, 0..=254) => {
            info!("IpAddress::ClassE({}.{}.{}.{})", ip.0, ip.1, ip.2, ip.3)
        }
        _ => info!("Unwanted Input"),
    }
}
/// main method to initializing and calling variables

fn main() {
    env_logger::init();
    let ip_1 = (10, 17, 89, 1);
    check_ip(ip_1);
    let ip_2 = (192, 168, 31, 1);
    check_ip(ip_2);
}