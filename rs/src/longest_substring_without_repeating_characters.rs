use std::collections::HashSet;

#[allow(dead_code)]
pub fn longest_substring_without_repeating_characters(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    if s.chars().count() == 1 {
        return 1;
    }

    let mut result = 0;
    let mut _l = 0;
    let cv: Vec<char> = s.chars().collect();

    let length = s.chars().count();
    let mut hash_set: HashSet<char> = HashSet::new();

    for i in 0..length - 1 {
        if i != 0 {
            hash_set.remove(&cv[i - 1]);
        }

        while _l < length {
            if hash_set.contains(&cv[_l]) {
                break;
            }

            hash_set.insert(cv[_l]);
            _l += 1;
        }

        result = result.max(hash_set.len());
    }
    result as i32
}

#[test]
fn internal() {
    let len = longest_substring_without_repeating_characters("abcabcbb".to_string());
    assert_eq!(len, 3);

    let len = longest_substring_without_repeating_characters("bbbbb".to_string());
    assert_eq!(len, 1);

    let len = longest_substring_without_repeating_characters("pwwkew".to_string());
    assert_eq!(len, 3);
}
