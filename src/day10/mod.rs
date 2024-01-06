mod helpers;
mod polygon;

use crate::day10::helpers::{
    find_point, points_inside_polygon, shoelace_formula, translate_symbols,
};
use crate::day10::polygon::{Point, Polygon};
use crate::helpers::is_debug;
use std::collections::HashMap;
use std::fs;

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
        cur = cur
            + if next_is_prev || next_is_oob {
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
