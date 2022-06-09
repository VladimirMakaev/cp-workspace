use std::{collections::VecDeque, fmt::Debug};

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

    pub fn orientation(a: &Point, b: &Point, c: &Point) -> PointOrientation {
        let x1 = a.x - b.x;
        let y1 = a.y - b.y;
        let x2 = c.x - b.x;
        let y2 = c.y - b.y;
        match (x1 * y2 - y1 * x2).cmp(&0) {
            std::cmp::Ordering::Less => PointOrientation::Left,
            std::cmp::Ordering::Equal => PointOrientation::Straight,
            std::cmp::Ordering::Greater => PointOrientation::Right,
        }
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
