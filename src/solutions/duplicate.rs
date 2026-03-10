use std::collections::HashMap;

pub fn has_duplicate(words: Vec<&str>) -> bool {
    let mut hm: HashMap<String, usize> = HashMap::new();
    for (_, v) in words.iter().enumerate() {
        let hm_word = hm.contains_key(*v);
        if hm_word {
            return true;
        } else {
            hm.insert(v.to_string(), 1);
        }
    }
    false
}

// let words = vec!["tokhir", "tokhir", "akmal"];
// let r = has_duplicate(words);
// println!("{r}")
