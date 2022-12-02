use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn fold_once(points: HashSet<Point>, fold_y: bool, fold_n: usize) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();
    for p in points {
        if fold_y && p.y > fold_n {
            let new_y = fold_n.abs_diff(p.y.abs_diff(fold_n));
            result.insert(Point { x: p.x, y: new_y });
        } else if !fold_y && p.x > fold_n {
            let new_x = fold_n.abs_diff(p.x.abs_diff(fold_n));
            result.insert(Point { x: new_x, y: p.y });
        } else {
            result.insert(p);
        }
    }
    result
}

fn format_points(points: HashSet<Point>) -> String {
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;
    let mut v: Vec<Vec<String>> = (0..=max_y)
        .map(|_| (0..=max_x).map(|_| ".".to_string()).collect())
        .collect();
    for p in points {
        v[p.y][p.x] = "*".to_string();
    }
    let mut result = String::new();
    for row in v {
        for value in row {
            result += &value;
        }
        result += "\n";
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_common::{get_data, split_lines};
    const PATH: &str = "inputs/day13.txt";
    const EXAMPLE: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    fn setup_data(data: Vec<String>) -> (HashSet<Point>, Vec<(bool, usize)>) {
        let mut points: HashSet<Point> = HashSet::new();
        let mut folds: Vec<(bool, usize)> = vec![];
        for row in data {
            if row.contains(',') {
                let mut split = row.split(',');
                let (x, y): (usize, usize) = (
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                );
                points.insert(Point { x, y });
            } else if row.contains('=') {
                let mut split = row.split_ascii_whitespace().last().unwrap().split('=');
                let (axis, num): (bool, usize) = (
                    split.next().unwrap().contains('y'),
                    split.next().unwrap().parse().unwrap(),
                );
                folds.push((axis, num));
            }
        }
        (points, folds)
    }

    #[test]
    fn example_1() {
        let (points, foldings) = setup_data(split_lines(EXAMPLE));
        let (fold_y, fold_n) = foldings.first().unwrap();
        let result = fold_once(points, *fold_y, *fold_n);
        assert_eq!(result.len(), 17);
    }

    #[test]
    fn example_2() {
        let (mut points, foldings) = setup_data(split_lines(EXAMPLE));
        for (fold_y, fold_n) in foldings {
            points = fold_once(points, fold_y, fold_n);
        }
        let result = format_points(points);
        let expected = "
        *****
        *...*
        *...*
        *...*
        *****";
        assert_eq!(result, split_lines(expected).join("\n"));
    }

    #[test]
    fn task_1() {
        let (points, foldings) = setup_data(get_data(PATH).unwrap());
        let (fold_y, fold_n) = foldings.first().unwrap();
        let result = fold_once(points, *fold_y, *fold_n);
        assert_eq!(result.len(), 684);
    }

    #[test]
    fn task_2() {
        let (mut points, foldings) = setup_data(get_data(PATH).unwrap());
        for (fold_y, fold_n) in foldings {
            points = fold_once(points, fold_y, fold_n);
        }
        let result = format_points(points);
        let expected = "
        ..**.***..****.***..*.....**..*..*.*..*
        ...*.*..*....*.*..*.*....*..*.*.*..*..*
        ...*.*..*...*..***..*....*....**...****
        ...*.***...*...*..*.*....*.**.*.*..*..*
        *..*.*.*..*....*..*.*....*..*.*.*..*..*
        .**..*..*.****.***..****..***.*..*.*..*";
        assert_eq!(result, split_lines(expected).join("\n"));
    }
}
