use std::{cmp::Ordering, collections::VecDeque, fmt::Debug};

#[derive(Clone, PartialEq, Eq, Ord, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PointOrientation {
    Left,
    Right,
    Straight,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Self { x, y }
    }
    /// Cross product between AB and CD
    pub fn cross_product(a: &Point, b: &Point, c: &Point, d: &Point) -> isize {
        let x1 = b.x - a.x;
        let y1 = b.y - a.y;
        let x2 = d.x - c.x;
        let y2 = d.y - c.y;
        x1 * y2 - y1 * x2
    }

    pub fn distance_to_edge(&self, edge_l: &Point, edge_r: &Point) -> f64 {
        let square = Point::cross_product(self, edge_l, self, edge_r).abs();
        return square as f64 / Point::distance(edge_l, edge_r);
    }

    pub fn orientation(a: &Point, b: &Point, c: &Point) -> PointOrientation {
        match Point::cross_product(&b, &a, &b, &c).cmp(&0) {
            std::cmp::Ordering::Less => PointOrientation::Left,
            std::cmp::Ordering::Equal => PointOrientation::Straight,
            std::cmp::Ordering::Greater => PointOrientation::Right,
        }
    }

    pub fn distance_sqr(lhr: &Point, rhr: &Point) -> usize {
        ((lhr.x - rhr.x) * (lhr.x - rhr.x)) as usize + ((lhr.y - rhr.y) * (lhr.y - rhr.y)) as usize
    }

    pub fn distance(lhr: &Point, rhr: &Point) -> f64 {
        (Point::distance_sqr(lhr, rhr) as f64).sqrt()
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

fn min_fl(left: f64, right: f64) -> f64 {
    match left.partial_cmp(&right) {
        Some(Ordering::Less) => left,
        Some(Ordering::Greater) => right,
        _ => left,
    }
}

pub fn diameter(convex_hull: &Vec<Point>) -> f64 {
    if convex_hull.len() < 2 {
        return 0.0;
    }
    if convex_hull.len() == 2 {
        return Point::distance(&convex_hull[0], &convex_hull[1]);
    }

    let size = convex_hull.len();

    fn next(index: usize, size: usize) -> usize {
        let res = index + 1;
        if res == size {
            return 0;
        }
        return res;
    }

    let mut left = 0;
    let (mut right, _) = convex_hull
        .iter()
        .enumerate()
        .max_by(|(i1, p1), (i2, p2)| p1.cmp(&p2))
        .unwrap();

    let mut min: f64 = f64::MAX;
    let start_right = right;

    loop {
        match Point::cross_product(
            &convex_hull[left],
            &convex_hull[next(left, size)],
            &convex_hull[right],
            &convex_hull[next(right, size)],
        )
        .cmp(&0)
        {
            std::cmp::Ordering::Less => {
                min = min_fl(
                    min,
                    convex_hull[right]
                        .distance_to_edge(&convex_hull[left], &convex_hull[next(left, size)]),
                );
                left = next(left, size);
            }
            _ => {
                min = min_fl(
                    min,
                    convex_hull[left]
                        .distance_to_edge(&convex_hull[right], &convex_hull[next(right, size)]),
                );
                right = next(right, size);
            }
        }

        if left == 0 && right == start_right {
            break;
        }
    }

    return min;
}

pub fn convex_hull(points: &Vec<Point>) -> Vec<Point> {
    let mut sorted = points.clone();
    if points.len() <= 3 {
        return sorted;
    }
    sorted.sort();
    let mut left_or_right: Vec<Option<bool>> = vec![None; points.len()];
    let mut left_set: VecDeque<usize> = VecDeque::new();
    let mut right_set: VecDeque<usize> = VecDeque::new();

    for next in 0..points.len() {
        loop {
            if left_set.len() < 2 {
                break;
            }

            match (left_set.pop_back(), left_set.pop_back()) {
                (Some(b), Some(a)) => {
                    match Point::orientation(&sorted[a], &sorted[b], &sorted[next]) {
                        PointOrientation::Right => {
                            left_set.push_back(a);
                        }
                        _ => {
                            left_set.push_back(a);
                            left_set.push_back(b);
                            break;
                        }
                    }
                }
                (_, _) => {
                    break;
                }
            }
        }
        left_set.push_back(next);
    }

    for left in left_set.into_iter() {
        left_or_right[left] = Some(true);
    }

    for next in 0..points.len() {
        if left_or_right[next].is_some() && next != 0 && next != points.len() - 1 {
            continue;
        }
        loop {
            if right_set.len() < 2 {
                break;
            }
            match (right_set.pop_back(), right_set.pop_back()) {
                (Some(b), Some(a)) => {
                    match Point::orientation(&sorted[a], &sorted[b], &sorted[next]) {
                        PointOrientation::Left => {
                            right_set.push_back(a);
                        }
                        _ => {
                            right_set.push_back(a);
                            right_set.push_back(b);
                            break;
                        }
                    }
                }
                (_, _) => {
                    break;
                }
            }
        }
        right_set.push_back(next);
    }
    for left in right_set.into_iter() {
        if left_or_right[left].is_none() {
            left_or_right[left] = Some(false);
        }
    }

    let mut result = Vec::new();

    for i in 0..sorted.len() {
        if left_or_right[i] == Some(true) {
            result.push(sorted[i].clone());
        }
    }
    for i in (0..sorted.len()).rev() {
        if left_or_right[i] == Some(false) {
            result.push(sorted[i].clone());
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = Point::new(-1, 0);
        let b = Point::new(-1, -1);
        let c = Point::new(1, 1);
        assert_eq!(Point::orientation(&a, &b, &c), PointOrientation::Left);
    }

    #[test]
    fn test2() {
        let a = Point::new(-1, -1);
        let b = Point::new(1, 1);
        let c = Point::new(2, -2);
        assert_eq!(Point::orientation(&a, &b, &c), PointOrientation::Right);
    }

    #[test]
    fn test_convex_hull_1() {
        let points = vec![
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(2, 0),
            Point::new(2, 4),
            Point::new(3, 2),
            Point::new(3, 3),
            Point::new(4, 2),
        ];
        let result = convex_hull(&points);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_distance() {
        let points = vec![Point::new(0, 0), Point::new(10, 0), Point::new(0, 10)];
        assert_eq!(points[0].distance_to_edge(&points[1], &points[2]), 1.0);
    }

    #[test]
    fn test_diameter_1() {
        let points = vec![
            Point::new(0, 0),
            Point::new(10, 0),
            Point::new(0, 10),
            Point::new(8, 6),
            Point::new(4, 8),
        ];

        assert_eq!(diameter(&convex_hull(&points)), 8.94427190999916)
    }

    struct Solution {}

    impl Solution {
        pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let points = trees
                .iter()
                .map(|t| Point::new(t[0] as isize, t[1] as isize))
                .into_iter()
                .collect::<Vec<_>>();

            let convex_hull = convex_hull(&points);
            convex_hull
                .iter()
                .map(|point| vec![point.x as i32, point.y as i32])
                .collect()
        }
    }
}
