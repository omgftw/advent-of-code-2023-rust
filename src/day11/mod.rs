use std::{fs, vec};
use std::ops::Sub;

#[cfg(test)]
mod tests;

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

fn expand_space(galaxy: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    // let mut new_galaxy = Vec::new();
    let mut empty_rows = (0..galaxy.len()).collect::<Vec<usize>>();
    let mut empty_cols = (0..galaxy[0].len()).collect::<Vec<usize>>();
    // iterate through rows
    for (y, _) in galaxy.iter().enumerate() {
        // iterate through cols
        for (x, _) in galaxy[y].iter().enumerate() {
            if galaxy[y][x] == '#' {
                //remove current x and y from empty rows and cols
                empty_rows.retain(|&i| i != y);
                empty_cols.retain(|&i| i != x);
            }
        }
    }

    let mut new_galaxy = galaxy.clone();
    empty_rows.reverse();
    empty_cols.reverse();
    // duplicate empty rows and cols
    for row in empty_rows.iter() {
        new_galaxy.insert(*row, vec!['.'; galaxy[0].len()]);
    }
    for col in empty_cols.iter() {
        for row in new_galaxy.iter_mut() {
            row.insert(*col, '.');
        }
    }

    new_galaxy
}

fn find_galaxies(galaxy: &Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxy_points = Vec::new();
    let mut cur_galaxy = 1;
    // iterate through rows
    for (y, _) in galaxy.iter().enumerate() {
        // iterate through cols
        for (x, _) in galaxy[y].iter().enumerate() {
            if galaxy[y][x] == '#' {
                galaxy_points.push(Point { id: cur_galaxy, x: x as isize, y: y as isize });
                cur_galaxy += 1;
            }
        }
    }

    galaxy_points
}

pub fn day11(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day11/data/main.txt").unwrap());
    let mut galaxy = Vec::new();
    for line in data.lines() {
        let line = line.chars().collect::<Vec<char>>();
        galaxy.push(line);
    }

    let galaxy = expand_space(&mut galaxy);
    let galaxy_points = find_galaxies(&galaxy);

    let mut sum = 0;
    for (i, point) in galaxy_points.iter().enumerate() {
        for j in i+1..galaxy_points.len() {
            let other_point = &galaxy_points[j];
            let diff = point - other_point;
            let x = diff.x as i32;
            let y = diff.y as i32;
            let diff = x.abs() + y.abs();
            sum += diff;
        }
    }

    // for row in galaxy.iter() {
    //     for col in row.iter() {
    //         print!("{}", col);
    //     }
    //     println!();
    // }

    (sum, 0)
}