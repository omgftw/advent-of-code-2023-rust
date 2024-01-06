#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Point {
            x: tuple.0 as isize,
            y: tuple.1 as isize,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point {
            x: tuple.0 as isize,
            y: tuple.1 as isize,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

pub struct Polygon {
    pub points: Vec<Point>,
}

impl Polygon {
    pub fn contains(&self, point: &Point) -> bool {
        let mut count = 0;
        let mut j = self.points.len() - 1;
        for i in 0..self.points.len() {
            let pi = &self.points[i];
            let pj = &self.points[j];
            if ((pi.y > point.y) != (pj.y > point.y))
                && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                count += 1;
            }
            j = i;
        }
        count % 2 == 1
    }
}
