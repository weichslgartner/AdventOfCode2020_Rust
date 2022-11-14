use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let my_str = include_str!("../input_03.txt");
    let (trees, limit) = parse_input(my_str);
    let slopes = vec![Point { x: 3, y: 1 }];
    let slope_mult = count_trees(&trees, &limit, slopes);
    println!("Part 1: {slope_mult}");
    let slopes = vec![Point { x: 1, y: 1 }, Point { x: 3, y: 1 }, Point { x: 5, y: 1 }, Point { x: 7, y: 1 }, Point { x: 1, y: 2 }];
    let slope_mult = count_trees(&trees, &limit, slopes);
    println!("Part 2: {slope_mult}");
}

fn count_trees(trees: &HashSet<Point>, limit: &Point, slopes: Vec<Point>) -> i32 {
    let mut slope_mult = 1;
    for slope in slopes.into_iter() {
        let mut x = 0;
        let mut y = 0;
        let mut tree_count = 0;
        while y <= limit.y {
            if trees.contains(&Point { x, y }) {
                tree_count += 1;
            }
            x += slope.x;
            if limit.x != 0 {
                x = x % limit.x;
            }
            y += slope.y;
        }
        slope_mult *= tree_count;
    }
    slope_mult
}

fn parse_input(my_str: &str) -> (HashSet<Point>, Point) {
    let mut x = 0;
    let mut y = 0;
    let mut trees = HashSet::new();
    for c in my_str.chars().into_iter() {
        match c {
            '#' => {
                trees.insert(Point { x, y });
                x += 1;
            }
            '.' => x += 1,
            '\n' => {
                y += 1;
                x = 0;
            }
            _ => {}
        }
    }
    let limit = Point { x, y };
    (trees, limit)
}
