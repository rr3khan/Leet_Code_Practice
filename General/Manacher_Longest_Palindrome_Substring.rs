struct Solution;
use std::cmp;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let size = chars.len();

        // Preprocess the string
        let mut post_processed_string: Vec<char> = vec!['^'];

        for index in 0..size {
            post_processed_string.push('#');
            post_processed_string.push(chars[index]);
        }

        post_processed_string.push('#');
        post_processed_string.push('$');

        let post_processed_size = post_processed_string.len();

        let mut longest_odd_palindrome = vec![0; post_processed_size];
        let mut center = 0;
        let mut rightmost_boundary = 0;

        for index in 1..post_processed_size - 1 {
            let mirror = 2 * center - index;

            if rightmost_boundary > index {
                longest_odd_palindrome[index] =
                    cmp::min(rightmost_boundary - index, longest_odd_palindrome[mirror])
            }
            while post_processed_string[index + (1 + longest_odd_palindrome[index])]
                == post_processed_string[index - (1 + longest_odd_palindrome[index])]
            {
                longest_odd_palindrome[index] += 1;
            }
            if index + longest_odd_palindrome[index] > rightmost_boundary {
                center = index;
                rightmost_boundary = index + longest_odd_palindrome[index];
            }
        }

        // Find the maximum value in the longest_odd_palindrome array
        let (center, max_len) = longest_odd_palindrome
            .iter()
            .enumerate()
            .max_by_key(|&(_, len)| len)
            .unwrap();

        // Extract the corresponding substring from the input string
        let start = (center - max_len) / 2;
        let end = start + max_len;
        let palindrome: String = chars[start..end].iter().collect();

        return palindrome;
    }
}


fn main() {
    let sol: Solution = Solution;
    println!("soln {}", Solution::longest_palindrome("bdfgirafarigaaaaajaba".to_string()));
}