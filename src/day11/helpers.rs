use crate::day11::Point;

pub fn find_empty_rows_and_cols(galaxy: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
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

    (empty_rows, empty_cols)
}

pub fn expand_space(
    galaxies: &[Point],
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_size: isize,
) -> Vec<Point> {
    let mut galaxies = galaxies.to_vec();
    for galaxy in galaxies.iter_mut() {
        let empty_rows_before = empty_rows
            .iter()
            .filter(|&i| *i < galaxy.y as usize)
            .count();
        let empty_cols_before = empty_cols
            .iter()
            .filter(|&i| *i < galaxy.x as usize)
            .count();

        galaxy.x += empty_cols_before as isize * expansion_size;
        galaxy.y += empty_rows_before as isize * expansion_size;
    }

    galaxies
}

pub fn find_galaxies(galaxy: &[Vec<char>]) -> Vec<Point> {
    let mut galaxy_points = Vec::new();
    let mut cur_galaxy = 1;
    // iterate through rows
    for (y, _) in galaxy.iter().enumerate() {
        // iterate through cols
        for (x, _) in galaxy[y].iter().enumerate() {
            if galaxy[y][x] == '#' {
                galaxy_points.push(Point {
                    id: cur_galaxy,
                    x: x as isize,
                    y: y as isize,
                });
                cur_galaxy += 1;
            }
        }
    }

    galaxy_points
}

pub fn get_galaxies_distance_sum(galaxy_points: &[Point]) -> i64 {
    let mut sum = 0;
    for (i, point) in galaxy_points.iter().enumerate() {
        for (_, other_point) in galaxy_points.iter().enumerate().skip(i + 1) {
            let diff = point - other_point;
            let x = diff.x as i64;
            let y = diff.y as i64;
            let diff = x.abs() + y.abs();
            sum += diff;
        }
    }

    sum
}