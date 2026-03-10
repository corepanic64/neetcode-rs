mod solutions;
use solutions::anagram::is_anagram;

fn main() {
    let s1 = "jar";
    let s2 = "jam";
    let r = is_anagram(s1, s2);
    println!("result: {r}")
}
