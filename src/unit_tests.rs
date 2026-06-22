#[cfg(test)]
mod tests {
    use crate::solution::Solution as sol;

    #[test]
    fn int_to_roman_exp1() {
        assert_eq!(
            sol::roman_to_int("III".to_string()),
            3
        );
    }

    #[test]
    fn int_to_roman_exp2() {
        assert_eq!(
            sol::roman_to_int("LVIII".to_string()), 
            58
        );
    }

    #[test]
    fn int_to_roman_exp3() {
        assert_eq!(
            sol::roman_to_int("MCMXCIV".to_string()), 
            1994
        );
    }

}