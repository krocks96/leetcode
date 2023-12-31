struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs{
            let mut v = s.as_bytes().to_vec();
            v.sort();
            map.entry(v).or_insert(Vec::new()).push(s);
        }
        // 最初に書いた方法
        // let mut result: Vec<Vec<String>> = Vec::new();
        // for value in map.values(){
        //     result.push(value.to_vec());
        // }
        // return result
        map.into_iter().map(|(_, v)| v).collect()
    }
}

fn main() {
    let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"].iter().map(|s| s.to_string()).collect();
    let result = Solution::group_anagrams(strs);
    println!("{:?}", result);
}