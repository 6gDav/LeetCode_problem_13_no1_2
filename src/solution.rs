pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s2 = s.to_uppercase();
        let chars: Vec<char> = s2.chars().collect();

        let mut total = 0;
        let mut parev_value = 0;

        for i in (0..s2.len()).rev() {
            let current_value = match chars[i] {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if current_value < parev_value {
                total -= current_value;
            } else {
                total += current_value;
            }

            parev_value = current_value;
        }
        total
    }
}
