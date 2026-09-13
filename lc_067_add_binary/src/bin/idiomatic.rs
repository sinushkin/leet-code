/*
https://leetcode.com/problems/add-binary/description/

Типичное решение в стиле, который часто встречается в обсуждениях на
LeetCode:
- оба числа читаются с конца через `.bytes().rev()` — без ручных счётчиков
  и деления на "длинную"/"короткую" строку, как в src/main.rs;
- цикл идёт, пока есть цифры хотя бы в одной из строк ИЛИ остался перенос:
  `if da.is_none() && db.is_none() && carry == 0 { break; }`;
- на каждом шаге: `sum = carry + digit_a + digit_b` (0, если итератор уже
  закончился), новая цифра — `sum % 2`, новый перенос — `sum / 2`;
- результат копится в Vec через push (не push_front), в конце — один
  reverse() и сборка в String.
*/
struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_iter = a.bytes().rev();
        let mut b_iter = b.bytes().rev();
        let mut carry = 0u8;
        let mut result = Vec::new();

        loop {
            let da = a_iter.next();
            let db = b_iter.next();
            if da.is_none() && db.is_none() && carry == 0 {
                break;
            }
            // sum лежит в диапазоне 0..=3 (два бита 0/1 плюс carry 0/1):
            // sum % 2 — цифра результата (остаток от деления на 2),
            // sum / 2 — новый перенос (целая часть от деления на 2).
            let sum = carry + da.map_or(0, |c| c - b'0') + db.map_or(0, |c| c - b'0');
            result.push(b'0' + sum % 2);
            carry = sum / 2;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    println!("{}", Solution::add_binary("11".to_string(), "1".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: &str) {
        assert_eq!(Solution::add_binary(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn example_1() {
        check("11", "1", "100");
    }

    #[test]
    fn example_2() {
        check("1010", "1011", "10101");
    }

    #[test]
    fn single_zeros() {
        check("0", "0", "0");
    }

    #[test]
    fn single_bits_with_carry() {
        check("1", "1", "10");
    }

    #[test]
    fn carry_propagates_through_all_ones() {
        check("111", "1", "1000");
    }

    #[test]
    fn equal_length_all_ones() {
        check("1111", "1111", "11110");
    }
}
