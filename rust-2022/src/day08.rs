use std::collections::HashMap;

use itertools::Itertools;

pub fn part1() -> usize {
    let input = include_str!("../../data/2022/08.txt");

    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (column, height_char) in line.chars().enumerate() {
            let height = height_char.to_digit(10).unwrap();
            grid.insert((row as i32, column as i32), height);
        }
    }

    //need to consider trees farther over - not just siblings
    let mut visible_trees: usize = 0;
    for (coordinate, height) in grid.iter() {
        if is_visible_along_line(
            coordinate,
            |&(row, column)| (row, column - 1),
            &grid,
            *height,
        ) || is_visible_along_line(
            coordinate,
            |&(row, column)| (row, column + 1),
            &grid,
            *height,
        ) || is_visible_along_line(
            coordinate,
            |&(row, column)| (row - 1, column),
            &grid,
            *height,
        ) || is_visible_along_line(
            coordinate,
            |&(row, column)| (row + 1, column),
            &grid,
            *height,
        ) {
            visible_trees += 1;
        }
    }

    visible_trees
}

fn is_visible_along_line(
    start_coordinate: &(i32, i32),
    next_coordinate: impl Fn(&(i32, i32)) -> (i32, i32),
    grid: &HashMap<(i32, i32), u32>,
    tree_height: u32,
) -> bool {
    let coordinate = next_coordinate(start_coordinate);
    match grid.get(&coordinate) {
        Some(&height) => {
            if height >= tree_height {
                false
            } else {
                is_visible_along_line(&coordinate, next_coordinate, grid, tree_height)
            }
        }
        None => true,
    }
}

pub fn part2() -> usize {
    let input = include_str!("../../data/2022/08.txt");

    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (column, height_char) in line.chars().enumerate() {
            let height = height_char.to_digit(10).unwrap();
            grid.insert((row as i32, column as i32), height);
        }
    }

    //need to consider trees farther over - not just siblings
    let mut max_tree_score: usize = 0;
    for (coordinate, height) in grid.iter() {
        let coordinate_score = count_visible_along_line(
            coordinate,
            |&(row, column)| (row, column - 1),
            &grid,
            *height,
            0,
        ) * count_visible_along_line(
            coordinate,
            |&(row, column)| (row, column + 1),
            &grid,
            *height,
            0,
        ) * count_visible_along_line(
            coordinate,
            |&(row, column)| (row - 1, column),
            &grid,
            *height,
            0,
        ) * count_visible_along_line(
            coordinate,
            |&(row, column)| (row + 1, column),
            &grid,
            *height,
            0,
        );

        if coordinate_score > max_tree_score {
            max_tree_score = coordinate_score;
        }
    }

    max_tree_score
}

fn count_visible_along_line(
    start_coordinate: &(i32, i32),
    next_coordinate: impl Fn(&(i32, i32)) -> (i32, i32),
    grid: &HashMap<(i32, i32), u32>,
    tree_height: u32,
    accum: usize,
) -> usize {
    let coordinate = next_coordinate(start_coordinate);
    match grid.get(&coordinate) {
        Some(&height) => {
            if height >= tree_height {
                accum + 1
            } else {
                count_visible_along_line(&coordinate, next_coordinate, grid, tree_height, accum + 1)
            }
        }
        None => accum,
    }
}
