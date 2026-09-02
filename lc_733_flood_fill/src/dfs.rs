use std::vec;
struct SolutionDFS;
/*
* Здесь Vec работает как stack:

push() → положить
pop()  → забрать последний

Поэтому это DFS.
*/
impl SolutionDFS {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;

        if image.is_empty() {
            return image;
        }

        let height = image.len();
        let width = image[0].len();

        let mut stack: Vec<(usize, usize)> = vec![(sr as usize, sc as usize)];

        let target_color = image[sr as usize][sc as usize];

        if target_color == color {
            return image;
        }

        image[sr as usize][sc as usize] = color;

        while let Some((row_index, col_index)) = stack.pop() {
            if row_index > 0 && image[row_index - 1][col_index] == target_color {
                image[row_index - 1][col_index] = color;
                stack.push((row_index - 1, col_index));
            }

            if row_index + 1 < height && image[row_index + 1][col_index] == target_color {
                image[row_index + 1][col_index] = color;
                stack.push((row_index + 1, col_index));
            }

            if col_index > 0 && image[row_index][col_index - 1] == target_color {
                image[row_index][col_index - 1] = color;
                stack.push((row_index, col_index - 1));
            }

            if col_index + 1 < width && image[row_index][col_index + 1] == target_color {
                image[row_index][col_index + 1] = color;
                stack.push((row_index, col_index + 1));
            }
        }

        image
    }
}
