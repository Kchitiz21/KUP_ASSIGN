use async_std::task;
use log::*;
use std::time::Duration;

/// Asynchronous Function 'process1' transforms a block of code using future
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

///Asynchronous Function 'process2' holds for future to complete
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
/// Asynchronous Function 'relate' is used to async function
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
