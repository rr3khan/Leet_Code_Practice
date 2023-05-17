struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
    
        let chars: Vec<char> = s.chars().collect();
        let size = chars.len();
        
        println!("Size {}", size);
    
        let mut bool_array: Vec<Vec<bool>> =  vec![vec![false; size]; size];
        let mut max_palin_size = 0;
        let mut palin_start = 0;
        let mut palin_end = 0;
        
        for start in 0..size {
            bool_array[start][start] = true;
        }
        
        for start in 0..size-1 {
            if chars[start] == chars[start+1]{
                bool_array[start][start +1] = true;
                max_palin_size = 1;
                palin_start = start;
                palin_end = start + 1;
            }
        }
        
        for len in 3..=size {
            for start in 0..=size-len {
            let end  = start + len - 1;
                    if chars[start] ==chars[end] && bool_array[start + 1][end - 1] {
                        bool_array[start][end] = true;
                        if len > max_palin_size {
                            max_palin_size = len;
                            palin_start =start;
                            palin_end = end;
                        }
                    }
            }
                
        }
        
        let palindrome: String = chars[palin_start..=palin_end].iter().collect(); 
        
        return palindrome;
            
    }
}

fn main() {
    let sol: Solution = Solution;
    println!("soln {}", Solution::longest_palindrome("bdfgirafarigaaaaajaba".to_string()));
}