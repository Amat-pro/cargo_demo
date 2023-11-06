/// 快速排序 双指针
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let left: usize = 0;
    let right = len - 1;

    _quick_sort(arr, left, right);
}

fn _quick_sort<T: Ord>(arr: &mut [T], left: usize, right: usize) {
    if left < right {
        let p = partition(arr, left, right);
        if p > 0 {
            _quick_sort(arr, left, p - 1);
        }
        if p < right {
            _quick_sort(arr, p + 1, right);
        }
    }
}

/// partition
/// pivot = right
/// 所有>pivot的left向右移动
/// 所有<pivot的right向左移动
/// 返回i的位置
/// 10, 8, 4, 3, 1, 9, 2, 7, 5, 6
/// 0   1  2  3  4  5  6  7  8  9
/// pivot = 9
/// i = 0
/// j = 9
/// i                        j  j
/// 5, 8, 4, 3, 1, 9, 2, 7, 10, 6
///    i                 j
/// 5, 2, 4, 3, 1, 9, 8, 7, 10, 6
///    i          i-j               // i的位置=j-1的位置
/// 5, 2, 4, 3, 1, 6, 8, 7, 10, 9
///                i                // swap(i, pivot), 返回i的位置
fn partition<T: Ord>(arr: &mut [T], left: usize, right: usize) -> usize {
    // 选取right作为pivot
    let pivot = right;

    let mut i = left;
    let mut j = right;

    loop {
        while arr[i] < arr[pivot] { // 边界：i=pivot时退出
            i += 1;
        }
        while j > 0 && arr[j - 1] > arr[pivot] { // 边界：j > 0
            j -= 1;
        }
        if j == 0 || i >= j - 1 {
            break;
        } else if arr[i] == arr[j - 1] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j - 1);
        }
    }
    arr.swap(i, pivot);
    i
}
