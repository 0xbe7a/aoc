fn read_elves(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Cant parse digit") as u8)
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let grid = read_elves(input);
    let mut visible = Vec::new();

    for _ in 0..grid.len() {
        visible.push(vec![false; grid.len()])
    }

    let mut mark_visible = |mut highest, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)| {
        let mut new_visible: usize = 0;

        loop {
            let height = Some(grid[x as usize][y as usize]);

            if height > highest {
                if !visible[x as usize][y as usize] {
                    new_visible += 1;
                }
                visible[x as usize][y as usize] = true;
                highest = height;
            }

            (x, y) = (x + dx, y + dy);

            if 0 > x || x >= grid.len() as i32 || 0 > y || y >= grid.len() as i32 {
                break;
            }
        }
        new_visible
    };

    let mut total_visible = 0;
    let n = grid.len() as i32 - 1;

    for i in 0..grid.len() as i32 {
        total_visible += mark_visible(None, (i, 0), (0, 1));
        total_visible += mark_visible(None, (n, i), (-1, 0));
        total_visible += mark_visible(None, (i, n), (0, -1));
        total_visible += mark_visible(None, (0, i), (1, 0));
    }

    total_visible
}

pub fn part_two(input: &str) -> usize {
    let grid = read_elves(input);

    let trees_along_axis = |highest, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)| {
        let mut visible: usize = 0;

        loop {
            (x, y) = (x + dx, y + dy);

            if 0 > x || x >= grid.len() as i32 || 0 > y || y >= grid.len() as i32 {
                break;
            }

            let height = grid[y as usize][x as usize];

            visible += 1;

            if height >= highest {
                break;
            }
        }
        visible
    };

    let mut max_score = None;

    for x in 0..grid.len() {
        for y in 0..grid.len() {
            let height = grid[y][x];
            let coord = (x as i32, y as i32);
            let score = Some(
                trees_along_axis(height, coord, (1, 0))
                    * trees_along_axis(height, coord, (-1, 0))
                    * trees_along_axis(height, coord, (0, 1))
                    * trees_along_axis(height, coord, (0, -1)),
            );

            max_score = std::cmp::max(score, max_score);
        }
    }

    max_score.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 8);
    }
}
