use std::collections::HashMap;
use std::fs;
use crate::helpers::is_debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Point { x: tuple.0 as isize, y: tuple.1 as isize }
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0 as isize, y: tuple.1 as isize }
    }
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn contains(&self, point: &Point) -> bool {
        let mut count = 0;
        let mut j = self.points.len() - 1;
        for i in 0..self.points.len() {
            let pi = &self.points[i];
            let pj = &self.points[j];
            if ((pi.y > point.y) != (pj.y > point.y)) &&
                (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x) {
                count += 1;
            }
            j = i;
        }
        count % 2 == 1
    }
}


fn find_point(matrix: &[Vec<char>], c: char) -> Option<Point> {
    for (y, line) in matrix.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == c {
                return Some((x, y).into());
            }
        }
    }
    None
}

fn translate_symbols(s: &str) -> String {
    s.replace('|', "║")
        .replace('-', "═")
        .replace('F', "╔")
        .replace('7', "╗")
        .replace('L', "╚")
        .replace('J', "╝")
}

fn points_inside_polygon(pipes: &Polygon) -> Vec<Point> {
    // sort by y and then x
    let mut sorted_points = pipes.points.clone();
    sorted_points.sort_by(|a, b| {
        if a.y == b.y {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });

    // check only points within the polygon
    // let mut internal_count = 0;
    let mut inside_points = Vec::new();
    let min_y = sorted_points[0].y;
    let max_y = sorted_points[sorted_points.len() - 1].y;
    for y in min_y..=max_y {
        let pipe_line =  sorted_points.iter().filter(|p| p.y == y).collect::<Vec<&Point>>();
        let min_x = pipe_line[0].x;
        let max_x = pipe_line[pipe_line.len() - 1].x;
        for x in min_x..=max_x {
            if  sorted_points.contains(&(x, y).into()) {
                continue;
            }
            // raycasting implementation
            if pipes.contains(&(x, y).into()) {
                // internal_count += 1;
                inside_points.push((x, y).into());
            }
        }
    }

    inside_points
}

fn shoelace_formula(points: &[Point]) -> f64 {
    let mut area = 0.0;
    let n = points.len();

    // Calculate area using the shoelace formula
    for i in 0..n {
        let j = (i + 1) % n; // Wrap around to the first point
        area += points[i].x as f64 * points[j].y as f64;
        area -= points[j].x as f64 * points[i].y as f64;
    }

    area.abs() / 2.0
}

pub(crate) async fn day10(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day10/data/main.txt").unwrap());
    let data = translate_symbols(&data);

    let mut position_map: HashMap<char, [Point; 2]> = HashMap::new();
    position_map.insert('|', [(0, -1).into(), (0, 1).into()]);
    position_map.insert('║', [(0, -1).into(), (0, 1).into()]);
    position_map.insert('-', [(-1, 0).into(), (1, 0).into()]);
    position_map.insert('═', [(-1, 0).into(), (1, 0).into()]);
    position_map.insert('F', [(1, 0).into(), (0, 1).into()]);
    position_map.insert('╔', [(1, 0).into(), (0, 1).into()]);
    position_map.insert('7', [(-1, 0).into(), (0, 1).into()]);
    position_map.insert('╗', [(-1, 0).into(), (0, 1).into()]);
    position_map.insert('L', [(1, 0).into(), (0, -1).into()]);
    position_map.insert('╚', [(1, 0).into(), (0, -1).into()]);
    position_map.insert('J', [(-1, 0).into(), (0, -1).into()]);
    position_map.insert('╝', [(-1, 0).into(), (0, -1).into()]);

    let mut data = data.lines().collect::<Vec<&str>>();
    let mut matrix = vec![];
    for line in data.iter_mut() {
        let line = line.chars().collect::<Vec<char>>();
        matrix.push(line);
    }

    let start = find_point(&matrix, 'S');
    let start = start.unwrap();

    // let mut pipes: Vec<Point> = Vec::new();
    let mut pipes: Polygon = Polygon { points: Vec::new() };
    let mut cur = start;
    let mut prev = start;
    pipes.points.push(cur);

    // check bottom
    let positions = position_map[&matrix[(cur.y + 1) as usize][cur.x as usize]];
    let bottom_pos = (0, -1);
    if positions.contains(&(bottom_pos.into())) {
        cur = (cur.x, cur.y + 1).into();
        pipes.points.push(cur);
    } else {
        cur = (cur.x, cur.y - 1).into();
        pipes.points.push(cur);
    }
    loop {
        let cur_char = matrix[cur.y as usize][cur.x as usize];
        let positions = position_map.get(&cur_char);
        if positions.is_none() {
            continue;
        }
        let positions = positions.unwrap();
        let next_is_prev = cur + positions[0] == prev;
        let next_is_oob = (cur + positions[0]).y > (matrix.len() - 1_usize) as isize;
        prev = cur;
        cur = cur + if next_is_prev || next_is_oob  {
            positions[1]
        } else {
            positions[0]
        };
        if cur == start {
            break;
        }
        pipes.points.push(cur);
    }

    if is_debug() {
        // create a new matrix the same size as the original filled with spaces
        let mut new_matrix = vec![];
        for line in matrix.iter() {
            let mut new_line = vec![];
            new_line.resize(line.len(), ' ');
            new_matrix.push(new_line);
        }

        for pipe in pipes.points.clone() {
            let cur_char = matrix[pipe.y as usize][pipe.x as usize];
            new_matrix[pipe.y as usize][pipe.x as usize] = cur_char;
        }

        let points_in_polygon = points_inside_polygon(&pipes);
        for point in points_in_polygon {
            new_matrix[point.y as usize][point.x as usize] = 'X';
        }

        for line in new_matrix.iter() {
            println!("{}", line.iter().collect::<String>());
        }
    }

    let area = shoelace_formula(&pipes.points);
    let len = pipes.points.len();
    let len = ((len as f32 - 1_f32) / 2f32).floor();
    let area = area - len as f64;

    let val = pipes.points.len() as f32 / 2f32;
    (val.ceil() as i32, area as i32)
}
