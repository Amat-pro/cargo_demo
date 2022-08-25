pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut len = arr.len();

    while !sorted {
        sorted = true;
        for i in 0..len - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        len -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        //descending
        // 6  0-4
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        println!("{:?}",ve1);
    }
}
