use std::{collections::HashSet, error::Error};
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap_or("input.in".to_string());
    let input = utility::get_raw_text(&filename)?;

    let regions = extract_regions(input.trim());
    let mut sum1 = 0;
    let mut sum2 = 0;
    for region in regions {
        sum1 += region.price_perimeter();
        sum2 += region.price_sides();
    }

    println!("{}", sum1);
    println!("{}", sum2);

    Ok(())
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Clone, Copy)]
struct Point(usize, usize);

fn dfs(grid: &[Vec<char>], seen: &mut HashSet<Point>, pos: Point, id: char) -> Point {
    seen.insert(pos);
    let mut area = 1;
    let mut perimeter = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let Point(x, y) = pos;
    for (dx, dy) in DIRECTIONS {
        let nx = dx + x as isize;
        let ny = dy + y as isize;

        if (nx < 0 || ny < 0 || nx >= rows || ny >= cols) || grid[nx as usize][ny as usize] != id {
            perimeter += 1;
            continue;
        }

        let np = Point(nx as usize, ny as usize);
        if seen.contains(&np) {
            continue;
        }

        let Point(a, p) = dfs(grid, seen, np, id);
        area += a;
        perimeter += p;
    }

    Point(area, perimeter)
}

fn extract_regions(input: &str) -> Vec<Region> {
    let grid: Vec<Vec<char>> = utility::get_vec_from_txt(input)
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut seen = HashSet::new();
    let mut regions = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &id) in row.iter().enumerate() {
            let p = Point(i, j);
            if seen.contains(&p) {
                continue;
            }

            let Point(area, perimeter) = dfs(&grid, &mut seen, p, id);
            let r = Region::new(area, perimeter, 0);
            regions.push(r);
        }
    }
    regions
}

#[derive(Debug, Default)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

impl Region {
    fn new(area: usize, perimeter: usize, sides: usize) -> Self {
        Self {
            area,
            perimeter,
            sides,
        }
    }

    fn price_perimeter(&self) -> usize {
        self.area * self.perimeter
    }

    fn price_sides(&self) -> usize {
        self.area * self.sides
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SMALL_EXAMPLE: &str = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
    const LARGER_EXAMPLE: &str = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

    fn get_sum_perimeter(regions: &[Region]) -> usize {
        regions.iter().map(|r| r.price_perimeter()).sum()
    }

    fn get_sum_sides(regions: &[Region]) -> usize {
        regions.iter().map(|r| r.price_sides()).sum()
    }

    #[test]
    fn smaller_example1() {
        let regions = extract_regions(SMALL_EXAMPLE);
        assert_eq!(get_sum_perimeter(&regions), 772);
    }

    #[test]
    fn larger_example1() {
        let regions = extract_regions(LARGER_EXAMPLE);
        assert_eq!(get_sum_perimeter(&regions), 1930);
    }

    #[test]
    fn smaller_example2() {
        let regions = extract_regions(SMALL_EXAMPLE);
        assert_eq!(get_sum_sides(&regions), 436);
    }

    #[test]
    fn larger_example2() {
        let regions = extract_regions(LARGER_EXAMPLE);
        assert_eq!(get_sum_sides(&regions), 1206);
    }
}
