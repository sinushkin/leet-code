struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() == 0 || t.len() != s.len() {
            return false;
        }
        let mut bank = [0u16; 256];
        let left = s.as_bytes();
        for i in 0..left.len() {
            bank[left[i] as usize] += 1;
        }

        let mut bank_right = [0u16; 256];
        let right = t.as_bytes();
        for i in 0..right.len() {
            bank_right[right[i] as usize] += 1;
        }

        for i in 0..bank.len() {
            if bank[i] != bank_right[i] {
                return false;
            }
        }

        return true;
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

/*

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0i32; 26];

        for (a, b) in s.bytes().zip(t.bytes()) {
            count[(a - b'a') as usize] += 1;
            count[(b - b'a') as usize] -= 1;
        }

        count.iter().all(|&x| x == 0)
    }
}
*/
