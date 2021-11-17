use log::info;
fn merge(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let num1 = mid - left;
    let num2 = right - mid;
    let num3 = arr.clone();
    let num4 = arr.clone();
    let num5 = &num3[left..mid];
    let num6 = &num4[mid..right];

    let mut num7 = 0;
    let mut num8 = 0;
    let mut num9 = left;
    while num7 < num1 && num8 < num2 {
        if num5[num7] < num6[num8] {
            arr[num9] = num5[num7];
            num7 +=  1;
        } else {
            arr[num9] = num6[num8];
            num8 +=1;
        }
        num9 +=1;
    }
    while num7 < num1 {
        arr[num9] = num5[num7];
        num7 +=1;
        num9 +=1;
    }

    while num8 < num2 {
        arr[num9] = num6[num8];
        num8 +=1;
        num9 +=1;
    }
    arr
}

fn merge_sort(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

fn main() {
    env_logger::init();
    let mut arr: Vec<i32> = vec![94, 1, 119, -87, 50, 43, 48];
    arr = merge_sort(arr.clone(), 0, arr.len());
    info!("Sorted array is {:?}", arr);
}
