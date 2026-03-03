struct Solution;
/// Алгоритм делает так:
/// Идёт слева направо
/// Хранит минимальную цену покупки min_price
/// На каждом шаге проверяет:
/// profit = текущая_цена - min_price

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut min_price = i32::MAX;
        let mut max_profit = 0;
        /*
        В каждый момент времени мы:
        Считаем, что текущий день — это день продажи.
        Проверяем: если бы мы купили по минимальной цене ДО этого дня, сколько бы заработали?
         */
        for price in prices {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }
        max_profit
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_1() {
        let prices = vec![7,1,5,3,6,4];
        let expected = 5;
        let actual = Solution::max_profit(prices);
        assert_eq!(expected, actual);
    }
}