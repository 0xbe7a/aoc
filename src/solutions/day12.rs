use std::{
    cmp::max,
    collections::{BinaryHeap, HashMap},
};

type Coord = (usize, usize);

struct Input {
    grid: Vec<Vec<u8>>,
    zero_levels: Vec<Coord>,
    start: Coord,
    end: Coord,
    width: usize,
    height: usize,
}

struct SearchPoint {
    coords: Coord,
    cost: usize,
    estimate: usize,
}

impl SearchPoint {
    fn estimate_total(&self) -> usize {
        self.cost + self.estimate
    }
}

impl PartialEq for SearchPoint {
    fn eq(&self, other: &Self) -> bool {
        self.estimate_total() == other.estimate_total()
    }
}

impl Eq for SearchPoint {}

impl PartialOrd for SearchPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.estimate_total().partial_cmp(&self.estimate_total())
    }
}

impl Ord for SearchPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimate_total().cmp(&self.estimate_total())
    }
}

fn read_heightmap(input: &str) -> Input {
    let mut start = None;
    let mut end = None;

    let mut grid = Vec::new();
    let mut zero_levels = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut height_line = Vec::new();
        for (x, c) in line.char_indices() {
            let height = match c {
                'S' => {
                    start = Some((x, y));
                    0
                }
                'E' => {
                    end = Some((x, y));
                    25
                }
                x => x as u8 - 'a' as u8,
            };

            if height == 0 {
                zero_levels.push((x, y));
            }

            height_line.push(height);
        }

        grid.push(height_line)
    }

    let height = grid.len();
    let width = grid.first().unwrap().len();

    Input {
        grid,
        start: start.expect("No start point found"),
        end: end.expect("No end point found"),
        height,
        width,
        zero_levels,
    }
}

fn find_shortest(input: &Input, start_points: &[(usize, usize)]) -> usize {
    let mut frontier = BinaryHeap::new();
    let mut shortest_paths = HashMap::new();

    let estimate_remaining_cost = |point: (usize, usize), current_height: u8| {
        let l1 = point.0.abs_diff(input.end.0) + point.1.abs_diff(input.end.1);
        let rem_height = 25 - current_height;
        max(l1, rem_height as usize)
    };

    let new_point = |(x, y), (dx, dy)| {
        let (nx, ny): (i32, i32) = (x as i32 + dx, y as i32 + dy);
        if 0 > nx || nx >= input.width as i32 || 0 > ny || ny >= input.height as i32 {
            return None;
        }

        Some((nx as usize, ny as usize))
    };

    for start in start_points {
        frontier.push(SearchPoint {
            coords: *start,
            cost: 0,
            estimate: estimate_remaining_cost(*start, 0),
        });
    }

    loop {
        let SearchPoint { coords, cost, .. } = frontier.pop().expect("No points left");

        if coords == input.end {
            break cost;
        }

        //generate_new_canidate
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(point) = new_point(coords, dir) {
                let next_height = input.grid[point.1][point.0];
                let next_cost = cost + 1;
                let current_best_cost = shortest_paths.get(&point).unwrap_or(&usize::MAX);
                if next_height.saturating_sub(input.grid[coords.1][coords.0]) > 1
                    || next_cost >= *current_best_cost
                {
                    continue;
                }

                let estimate = estimate_remaining_cost(point, next_height);
                shortest_paths.insert(point, next_cost);

                frontier.push(SearchPoint {
                    coords: point,
                    cost: next_cost,
                    estimate,
                });
            }
        }

    }
}

pub fn part_one(input: &str) -> usize {
    let input = read_heightmap(input);
    find_shortest(&input, &[input.start])
}

pub fn part_two(input: &str) -> usize {
    let input = read_heightmap(input);
    find_shortest(&input, &input.zero_levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_one(&input), 31);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_two(&input), 29);
    }
}
