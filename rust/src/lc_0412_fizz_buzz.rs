pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut answer: Vec<String> = Vec::new();
    for i in 1..n+1 {
        if i % 3 == 0 && i % 5 == 0 {
            answer.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            answer.push("Fizz".to_string());
        } else if i % 5 == 0 {
            answer.push("Buzz".to_string());
        } else {
            answer.push(i.to_string());
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    // n = 3
    #[test]
    fn test_1() {
        let n = 3;
        assert_eq!(fizz_buzz(n), vec!["1", "2", "Fizz"]);
    }

    // n = 5
    #[test]
    fn test_2() {
        let n = 5;
        assert_eq!(fizz_buzz(n), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    // n = 15
    #[test]
    fn test_3() {
        let n = 15;
        assert_eq!(fizz_buzz(n), vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"])
    }

}