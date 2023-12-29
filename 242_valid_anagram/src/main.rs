use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let map1 = Solution::string_to_hashmap(s);
        let map2 = Solution::string_to_hashmap(t);
        return map1 == map2;
    }

    fn string_to_hashmap(s: String) -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        return map;
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    println!("{}", Solution::is_anagram(s, t));
}