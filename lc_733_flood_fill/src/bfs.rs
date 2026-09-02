use std::collections::VecDeque;
/*
* BFS — через VecDeque

Почти тот же код, но вместо стека — очередь:
Ключевая разница всего в этом:

// DFS
stack.push(...)
stack.pop()

против:

// BFS
queue.push_back(...)
queue.pop_front()

*BFS — Breadth-First Search
Поиск в ширину.
Сначала обработать всех непосредственных соседей, потом соседей этих соседей, потом следующий уровень.
*/
struct SolutionBFS;

impl SolutionBFS {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;

        if image.is_empty() {
            return image;
        }

        let height = image.len();
        let width = image[0].len();

        let target_color = image[sr as usize][sc as usize];

        if target_color == color {
            return image;
        }

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        queue.push_back((sr as usize, sc as usize));
        image[sr as usize][sc as usize] = color;

        while let Some((row_index, col_index)) = queue.pop_front() {
            if row_index > 0 && image[row_index - 1][col_index] == target_color {
                image[row_index - 1][col_index] = color;
                queue.push_back((row_index - 1, col_index));
            }

            if row_index + 1 < height && image[row_index + 1][col_index] == target_color {
                image[row_index + 1][col_index] = color;
                queue.push_back((row_index + 1, col_index));
            }

            if col_index > 0 && image[row_index][col_index - 1] == target_color {
                image[row_index][col_index - 1] = color;
                queue.push_back((row_index, col_index - 1));
            }

            if col_index + 1 < width && image[row_index][col_index + 1] == target_color {
                image[row_index][col_index + 1] = color;
                queue.push_back((row_index, col_index + 1));
            }
        }

        image
    }
}
