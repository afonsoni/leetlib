pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_chars = s.chars().collect::<Vec<char>>();
    let mut t_chars = t.chars().collect::<Vec<char>>();
    s_chars.sort();
    t_chars.sort();
    s_chars == t_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert_eq!(is_anagram(s, t), true);
    }

    #[test]
    fn test_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        assert_eq!(is_anagram(s, t), false);
    }
}