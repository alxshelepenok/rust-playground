struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let (mut result, mut previous) = (0, 0);

        for c in s.chars() {
            let current = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            result += match previous < current {
                true => current - previous * 2,
                false => current,
            };

            previous = current;
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
