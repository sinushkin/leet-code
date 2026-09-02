/*

Начни с пикселя с координатами sr, sc и измени его цвет на color.

Затем найди все пиксели, которые непосредственно прилегают к нему по стороне — сверху, снизу, слева или справа — и имеют тот же цвет, что был у исходного пикселя.

Измени цвет этих пикселей на color.

После этого продолжай проверять соседей уже изменённых пикселей и таким же образом изменяй их, если их цвет совпадает с исходным цветом стартового пикселя.

Продолжай, пока больше не останется подходящих соседних пикселей.

Верни получившееся изображение.*/

use std::collections::HashSet;

struct Solution;
type Position = (usize, usize);

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        if image.is_empty() {
            return image;
        }
        let mut processed_set: HashSet<Position> = HashSet::new();
        let mut todo_set: HashSet<Position> = HashSet::new();
        let height = image.len();
        let width = image[0].len();

        let mut row_index = sr as usize;
        let mut col_index = sc as usize;
        let target_color = image[row_index][col_index];
        image[row_index][col_index] = color;
        processed_set.insert((row_index, col_index));

        //заполняем куда бы нам надо пойти от центральной точки
        loop {
            //вверх
            if row_index > 0 {
                let position = (row_index - 1, col_index);
                if !processed_set.contains(&position)
                    && image[row_index - 1][col_index] == target_color
                {
                    todo_set.insert(position);
                    image[row_index - 1][col_index] = color;
                }
            }
            //лево
            if col_index > 0 {
                let position = (row_index, col_index - 1);

                if !processed_set.contains(&position)
                    && image[row_index][col_index - 1] == target_color
                {
                    todo_set.insert(position);
                    image[row_index][col_index - 1] = color;
                }
            }
            //низ
            if row_index < height - 1 {
                let position = (row_index + 1, col_index);

                if !processed_set.contains(&position)
                    && image[row_index + 1][col_index] == target_color
                {
                    todo_set.insert(position);
                    image[row_index + 1][col_index] = color;
                }
            }
            //право
            if col_index < width - 1 {
                let position = (row_index, col_index + 1);

                if !processed_set.contains(&position)
                    && image[row_index][col_index + 1] == target_color
                {
                    todo_set.insert(position);
                    image[row_index][col_index + 1] = color;
                }
            }
            processed_set.insert((row_index, col_index));

            if let Some(next) = todo_set.iter().next().copied() {
                row_index = next.0;
                col_index = next.1;
                todo_set.remove(&next);
            } else {
                break;
            }
        }
        image
    }
}

fn main() {
    let image = vec![vec![1i32, 1i32, 1i32], vec![1, 1, 0], vec![1, 0, 1]];
    let image = Solution::flood_fill(image, 1, 1, 2);
    println!("{:?}", image);
    let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
    let image = Solution::flood_fill(image, 0, 0, 0);
    println!("{:?}", image);
}
