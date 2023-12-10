use std::fs;

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Could not read file!");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines.lines() {
        grid.push(line.chars().collect())
    }

    let mut i_row = 0;
    let mut i_col;
    let mut i_start = None;
    let mut digits = Vec::new();
    let mut numbers = Vec::new();
    for row in grid.clone() {
        if i_start.is_some() {
            numbers.push((
                digits.iter().collect::<String>().parse::<usize>().unwrap(),
                i_row - 1,
                i_start.unwrap(),
                row.len() - 1,
            ));
        }
        digits.clear();
        i_start = None;
        i_col = 0;
        for element in row {
            if element.is_numeric() {
                digits.push(element);
                if i_start.is_none() {
                    i_start = Some(i_col)
                }
            }
            if !element.is_numeric() && i_start.is_some() {
                numbers.push((
                    digits.iter().collect::<String>().parse::<usize>().unwrap(),
                    i_row,
                    i_start.unwrap(),
                    i_col - 1,
                ));
                digits.clear();
                i_start = None;
            }
            i_col += 1;
        }
        i_row += 1;
    }

    let mut part_numbers = Vec::new();
    for (number, i_row, i_start, i_end) in numbers {
        let mut positions = Vec::new();
        if i_start > 0 {
            positions.push((i_row, i_start - 1));
        }
        if i_end < 139 {
            positions.push((i_row, i_end + 1));
        }
        let i_search_start = match i_start > 0 {
            true => i_start - 1,
            false => i_start,
        };
        let i_search_end = match i_end < grid.len() - 1 {
            true => i_end + 1,
            false => i_end,
        };
        if i_row > 0 {
            for i_col in i_search_start..i_search_end + 1 {
                positions.push((i_row - 1, i_col))
            }
        }
        if i_row < 139 {
            for i_col in i_search_start..i_search_end + 1 {
                positions.push((i_row + 1, i_col))
            }
        }
        for (i_row, i_col) in positions {
            if is_symbol(grid[i_row][i_col]) {
                part_numbers.push(number)
            }
        }
    }

    let mut total = 0;
    for part_number in part_numbers {
        total = total + part_number
    }
    println!("{total}")
}
