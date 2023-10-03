
use std::io;
use std::collections::HashSet;
struct Solution;
fn main() {

    //contains_duplicate
    // let nums = input_values();

    // match Solution::contains_duplicate(nums) {
    //     true => println!("Yes"),
    //     false => println!("No")
    // }
    
    let str = input_line();
    let other_str = input_line();
    
    match Solution::is_anagram(str, other_str) {
        true => println!("Yes"),
        false => println!("No")
    }
}

fn input_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn input_values() -> Vec<i32> {
    let values: Vec<i32> = input_line()
    .split_whitespace()
    .map(|q| q.parse().unwrap())
    .collect();
    values
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut note = HashSet::new(); // Create the hash set
        for num in nums.iter(){
            match note.contains(num) {
                true => return true,
                _ => {note.insert(num);}
            }
        }
        false
    }
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }

        let mut letters = [0_i32; 26];

        s.bytes().zip(t.bytes()).for_each(|(u, v)| {
            letters[(u - b'a') as usize] += 1;
            letters[(v - b'a') as usize] -= 1;
        });
        
        for cnt in letters {
            if cnt != 0 { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    }
    #[test]
    fn test_is_anagram() {
        assert_eq!(Solution::is_anagram(String::from("anagram"), String::from("nagaram")), true);
        assert_eq!(Solution::is_anagram(String::from("cargo"), String::from("gocar")), true);
        assert_eq!(Solution::is_anagram(String::from("rat"), String::from("car")), false);
    }
}