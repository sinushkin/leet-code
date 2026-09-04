struct SolutionRecursion;

impl SolutionRecursion {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;

        if image.is_empty() {
            return image;
        }

        let target_color = image[sr as usize][sc as usize];

        if target_color == color {
            return image;
        }

        Self::fill(&mut image, sr as usize, sc as usize, target_color, color);

        image
    }

    fn fill(
        image: &mut Vec<Vec<i32>>,
        row_index: usize,
        col_index: usize,
        target_color: i32,
        color: i32,
    ) {
        if image[row_index][col_index] != target_color {
            return;
        }

        image[row_index][col_index] = color;

        if row_index > 0 {
            Self::fill(image, row_index - 1, col_index, target_color, color);
        }

        if row_index + 1 < image.len() {
            Self::fill(image, row_index + 1, col_index, target_color, color);
        }

        if col_index > 0 {
            Self::fill(image, row_index, col_index - 1, target_color, color);
        }

        if col_index + 1 < image[0].len() {
            Self::fill(image, row_index, col_index + 1, target_color, color);
        }
    }
}
