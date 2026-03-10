pub fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().collect();
    chars1.sort();
    let mut s1 = String::from("");
    for c in chars1 {
        s1.push(c);
    }
    let mut chars2: Vec<char> = s2.chars().collect();
    chars2.sort();
    let mut s2 = String::from("");
    for c in chars2 {
        s2.push(c);
    }
    if s1 == s2 { return true } else { return false }
}
