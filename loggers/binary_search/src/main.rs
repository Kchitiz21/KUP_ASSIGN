use log::info;
use log::warn;
fn binary_search(item: &i32, arr: &[i32]) -> i32 {
    let mut idx_pos = -1;

    if arr.is_empty() {
        return idx_pos;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if &arr[mid] > item {
            right = mid - 1;
        } else if &arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if &arr[left] == item {
        idx_pos = left as i32;
        return idx_pos;
    } else {
        return idx_pos;
    }
}
fn main() {
    env_logger::init();
    let item = -17;
    let arr: [i32; 6] = [77, 25, -17, 1, 48, 7];
    let index = binary_search(&item, &arr);

    if index >= 0 {
        info!("Element found at position: {}", index);
    }
    else {
        warn!("Element not found");
    }
}
