pub fn number_of_steps(num: i32) -> i32 {
    let mut temp_num = num;  // mutable variable
    let mut steps: i32 = 0;
    while temp_num != 0 {
        if (temp_num % 2) == 0 {
            temp_num = temp_num / 2;
        } else {
            temp_num = temp_num - 1;
        }
        steps = steps + 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    // num = 14
    #[test]
    fn test_1() {
        let num = 14;
        assert_eq!(number_of_steps(num), 6);
    }

    // num = 8
    #[test]
    fn test_2() {
        let num = 8;
        assert_eq!(number_of_steps(num), 4);
    }

    // num = 123
    #[test]
    fn test_3() {
        let num = 123;
        assert_eq!(number_of_steps(num), 12);
    }
}
