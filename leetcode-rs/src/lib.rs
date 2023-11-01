mod lc_0001_two_sum;

#[cfg(test)]
mod tests {
    use crate::lc_0001_two_sum;

    #[test]
    fn two_sum() {
        let r = lc_0001_two_sum::two_sum(vec![2, 7, 11, 15], 9);
        println!("fn: two_sum, result: {:?}", r);
    }
}
