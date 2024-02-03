struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let prefix = &strs[0];

        for (i, c) in prefix.chars().enumerate() {
            for s in &strs[1..] {
                if i >= s.len() || s.chars().nth(i).unwrap() != c {
                    return prefix[..i].to_string();
                }
            }
        }

        prefix.to_string()
    }
}

fn main() {
    assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string());
    }
}
