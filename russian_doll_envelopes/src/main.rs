struct Solution {}

impl Solution {
    pub fn longest_increasing_subsequence<T: Ord + Clone>(input_array: &[T]) -> Vec<T> {
        let n = input_array.len();
        if n <= 1 {
            return input_array.to_vec();
        }

        let mut increasing_sequence: Vec<(T, usize)> = Vec::new();
        let mut previous = vec![0_usize; n];

        increasing_sequence.push((input_array[0].clone(), 1));
        for i in 1..n {
            let value = input_array[i].clone();
            if value > increasing_sequence.last().unwrap().0 {
                previous[i] = increasing_sequence.last().unwrap().1 - 1;
                increasing_sequence.push((value, i + 1));
                continue;
            }

            let change_position = increasing_sequence
                .binary_search(&(value.clone(), 0))
                .unwrap_or_else(|x| x);
            increasing_sequence[change_position] = (value, i + 1);
            previous[i] = match change_position {
                0 => i,
                other => increasing_sequence[other - 1].1 - 1,
            };
        }

        let mut out: Vec<T> = Vec::with_capacity(increasing_sequence.len());

        out.push(increasing_sequence.last().unwrap().0.clone());
        let mut current_index = increasing_sequence.last().unwrap().1 - 1;
        while previous[current_index] != current_index {
            current_index = previous[current_index];
            out.push(input_array[current_index].clone());
        }

        out.into_iter().rev().collect()
    }

    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });


        Solution::longest_increasing_subsequence(
            &envelopes.iter().map(|x| x[1]).collect::<Vec<i32>>(),
        ).len() as i32
    }
}

fn main() {
    assert_eq!(Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]), 1);
    }
}
