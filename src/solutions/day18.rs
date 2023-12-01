use rustc_hash::FxHashSet as HashSet;

use itertools::Itertools;

type Cubelet = (i16, i16, i16);

fn read_cubelets(input: &str) -> impl Iterator<Item = Cubelet> + '_ {
    input.lines().map(|line| {
        line.split(',')
            .map(|x| x.parse().expect("Cant parse num"))
            .collect_tuple()
            .expect("Cant parse cubelet")
    })
}

fn get_faces(cube: &Cubelet) -> impl Iterator<Item = Cubelet> + '_ {
    [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]
    .iter()
    .map(|d| (cube.0 + d.0, cube.1 + d.1, cube.2 + d.2))
}

pub fn part_one(input: &str) -> u32 {
    let cubelets: HashSet<Cubelet> = read_cubelets(input).collect();
    let mut faces = 0;

    for cube in cubelets.iter() {
        for face in get_faces(cube) {
            if !cubelets.contains(&face) {
                faces += 1;
            }
        }
    }

    faces
}

pub fn part_two(input: &str) -> u32 {
    let cubelets: HashSet<Cubelet> = read_cubelets(input).collect();
    let mut air = HashSet::default();
    let max_radius = cubelets
        .iter()
        .map(|(a, b, c)| a.abs() + b.abs() + c.abs())
        .max()
        .expect("No cubes")
        + 1;

    let mut exploring = Vec::new();

    for x in -max_radius..=max_radius {
        for y in -(max_radius - x.abs())..=(max_radius - x.abs()) {
            for z in [
                -(max_radius - x.abs() - y.abs()),
                max_radius - x.abs() - y.abs(),
            ] {
                air.insert((x, y, z));
                exploring.push((x, y, z));
            }
        }
    }

    while let Some(cube) = exploring.pop() {
        for side in get_faces(&cube) {
            if side.0.abs() + side.1.abs() + side.2.abs() < max_radius {
                if !cubelets.contains(&side) && !air.contains(&side) {
                    air.insert(side);
                    exploring.push(side);
                }
            }
        }
    }

    let mut faces = 0;

    for cube in cubelets.iter() {
        for face in get_faces(cube) {
            if !cubelets.contains(&face) && air.contains(&face) {
                faces += 1;
            }
        }
    }

    faces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 18);
        assert_eq!(part_one(&input), 64);
    }

    /* #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 18);
        assert_eq!(part_two(&input), 58);
    } */
}
