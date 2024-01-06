use crate::day10::polygon::{Point, Polygon};

pub fn find_point(matrix: &[Vec<char>], c: char) -> Option<Point> {
    for (y, line) in matrix.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == c {
                return Some((x, y).into());
            }
        }
    }
    None
}

pub fn translate_symbols(s: &str) -> String {
    s.replace('|', "║")
        .replace('-', "═")
        .replace('F', "╔")
        .replace('7', "╗")
        .replace('L', "╚")
        .replace('J', "╝")
}

pub fn points_inside_polygon(pipes: &Polygon) -> Vec<Point> {
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
        let pipe_line = sorted_points
            .iter()
            .filter(|p| p.y == y)
            .collect::<Vec<&Point>>();
        let min_x = pipe_line[0].x;
        let max_x = pipe_line[pipe_line.len() - 1].x;
        for x in min_x..=max_x {
            if sorted_points.contains(&(x, y).into()) {
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

pub fn shoelace_formula(points: &[Point]) -> f64 {
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
