pub mod template;

// Use this file to add helper functions and additional modules.

/// Find all neighboring indices in a 2D grid (with or without diagonals)
pub fn get_neighboring_indices_2d(
    i: usize,
    j: usize,
    &height: &usize,
    &width: &usize,
    diagonals: bool,
) -> Vec<(usize, usize)> {
    let offsets: Vec<(isize, isize)> = if diagonals {
        Vec::from([
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1), // normal
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1), // diags
        ])
    } else {
        Vec::from([(-1, 0), (1, 0), (0, -1), (0, 1)])
    };
    offsets
        .iter()
        .filter_map(|&(dx, dy)| {
            let nx = i as isize + dx;
            let ny = j as isize + dy;
            if nx >= 0 && nx < height as isize && ny >= 0 && ny < width as isize {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .collect()
}

/// Prettyprint a grid of characters
pub fn prettyprint_grid(grid: &[Vec<char>]) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

/// Small helper function to check if a coordinate is inside a grid
pub fn in_grid(i: isize, j: isize, height: isize, width: isize) -> bool {
    i >= 0 && i < height && j >= 0 && j < width
}

/// Returns the number of digits of a decimal number
pub fn count_digits(mut num: u64) -> u32 {
    let mut count = 0;
    if num == 0 {
        return 1;
    }
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}
