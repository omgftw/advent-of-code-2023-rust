use std::fs;
use std::ops::Sub;
use crate::day11::helpers::{expand_space, find_empty_rows_and_cols, find_galaxies, get_galaxies_distance_sum};

#[cfg(test)]
mod tests;
mod helpers;

#[derive(Debug, Clone, Copy)]
struct Point {
    id: i32,
    x: isize,
    y: isize,
}

impl<'a> Sub for &'a Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            id: self.id,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn day11(data: Option<String>, expansion_size: i32) -> i64 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day11/data/main.txt").unwrap());
    let mut galaxy = Vec::new();
    for line in data.lines() {
        let line = line.chars().collect::<Vec<char>>();
        galaxy.push(line);
    }

    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&galaxy);
    let orig_galaxy_points = find_galaxies(&galaxy);
    let galaxy_points = expand_space(
        &orig_galaxy_points,
        &empty_rows,
        &empty_cols,
        expansion_size as isize,
    );

    get_galaxies_distance_sum(&galaxy_points)
}
