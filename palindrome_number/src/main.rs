struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let (mut i, mut r) = (x, 0);

        while i != 0 {
            r = r * 10 + i % 10;
            i /= 10;
        }

        x == r
    }
}

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
