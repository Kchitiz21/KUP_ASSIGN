use async_std::task;
use log::*;
use std::time::Duration;

/// process1: It is used to transforms a block of code using future
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// there is no return type
pub async fn process1() {
    for i in 1..10 {
        info!("Process1 ");
        if i == 9 {
            task::sleep(Duration::from_secs(2)).await;
        }
    }
}

/// process2: It holds for future to complete.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// there is no return type
pub async fn process2() {
    for _i in 1..10 {
        info!("process2 ");
    }
}
/// relate: It is used to async function.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// there is no return type
pub async fn relate() {
    let first = process1();
    let second = process2();

    futures::join!(first, second);
}
