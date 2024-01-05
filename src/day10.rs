use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point(isize, isize);

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Point(tuple.0 as isize, tuple.1 as isize)
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point(tuple.0 as isize, tuple.1 as isize)
    }
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Point(tuple.0, tuple.1)
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
pub(crate) async fn day10(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/data/10.txt").unwrap());
    let mut connect_map = HashMap::new();
    connect_map.insert('|', ['F', '7', 'L', 'J']);
    connect_map.insert('-', ['F', '7', 'L', 'J']);
    connect_map.insert('F', ['|', '-', '7', 'L']);
    connect_map.insert('7', ['|', '-', 'F', 'J']);
    connect_map.insert('L', ['|', '-', 'F', 'J']);
    connect_map.insert('J', ['|', '-', '7', 'L']);

    let mut position_map: HashMap<char, [Point; 2]> = HashMap::new();
    position_map.insert('|', [(0, -1).into(), (0, 1).into()]);
    position_map.insert('-', [(-1, 0).into(), (1, 0).into()]);
    position_map.insert('F', [(1, 0).into(), (0, 1).into()]);
    position_map.insert('7', [(-1, 0).into(), (0, 1).into()]);
    position_map.insert('L', [(1, 0).into(), (0, -1).into()]);
    position_map.insert('J', [(-1, 0).into(), (0, -1).into()]);

    let mut data = data.lines().collect::<Vec<&str>>();
    let mut matrix = vec![];
    for line in data.iter_mut() {
        let line = line.chars().collect::<Vec<char>>();
        matrix.push(line);
    }

    let start = find_point(&matrix, 'S');
    let start = start.unwrap();

    let mut pipes: Vec<Point> = Vec::new();
    let mut cur = start;
    let mut prev = start;
    pipes.push(cur);

    // check bottom
    let positions = position_map[&matrix[(cur.1 + 1) as usize][cur.0 as usize]];
    let bottom_pos = (0, -1);
    if positions.contains(&(bottom_pos.into())) {
        cur = (cur.0, cur.1 + 1).into();
        pipes.push(cur);
    } else {
        cur = (cur.0, cur.1 - 1).into();
        pipes.push(cur);
    }
    loop {
        let cur_char = matrix[cur.1 as usize][cur.0 as usize];
        let positions = position_map.get(&cur_char);
        if positions.is_none() {
            continue;
        }
        let positions = positions.unwrap();
        let next_is_prev = cur + positions[0] == prev;
        let next_is_oob = (cur + positions[0]).1 > (matrix.len() - 1_usize) as isize;
        prev = cur;
        cur = cur + if next_is_prev || next_is_oob  {
            positions[1]
        } else {
            positions[0]
        };
        if cur == start {
            break;
        }
        pipes.push(cur);
    }

    let val = pipes.len() as f32 / 2f32;
    (val.ceil() as i32, 0)
}
