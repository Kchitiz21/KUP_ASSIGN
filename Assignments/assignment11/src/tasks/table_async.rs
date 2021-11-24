use async_std::task;
use log::*;
use std::time::Duration;

/// check: It is used to async function and print two tables.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// there is no return type.
pub async fn check() {
    let first_one = async {
        for initial in 0..10 {
            info!("2*{} = {} ", initial, 2 * initial);
            task::sleep(Duration::from_secs(3)).await;
        }
    };
    let second_one = async {
        for initial in 0..10 {
            info!("3*{} = {} ", initial, 3 * initial);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(first_one, second_one);
}
