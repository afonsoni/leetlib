pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let rm = ransom_note;
    let mut m = magazine;
    for c in rm.chars() {
        if let Some(i) = m.find(c) {
            m.remove(i);
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }

    #[test]
    fn test_2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        assert_eq!(can_construct(ransom_note, magazine), false);
    }

    #[test]
    fn test_3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        assert_eq!(can_construct(ransom_note, magazine), true);
    }
}