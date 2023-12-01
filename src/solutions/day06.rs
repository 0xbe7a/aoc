use std::simd::{LaneCount, Mask, Simd, SimdPartialEq, SupportedLaneCount};

pub fn index_simd<const WIDTH: usize>(input: &str, window: usize) -> Option<usize>
where
    LaneCount<WIDTH>: SupportedLaneCount,
{
    assert!(window <= WIDTH);
    let mut mask = [false; WIDTH];
    for idx in 1..window {
        mask[idx] = true;
    }

    let mask: Mask<i8, WIDTH> = Mask::from_array(mask);
    let mut buffer = Simd::from_array([0u8; WIDTH]);

    for (idx, char) in input.chars().enumerate() {
        buffer = buffer.rotate_lanes_right::<1>();
        let cmp_mask = Simd::splat(char as u8).simd_eq(buffer) & mask;
        let null_vec = Simd::splat(0);
        buffer = cmp_mask.select(null_vec, buffer);
        buffer[0] = char as u8;

        let duplicates = (null_vec.simd_eq(buffer) & mask).any();
        if !duplicates {
            return Some(idx + 1);
        }
    }
    None
}

pub fn _index_popcnt(input_str: &str, window: usize) -> Option<usize> {
    let mut buffer = 0u32;
    let mask = |c| 1u32 << (c - 'a' as u8);
    let input = input_str.as_bytes();
    
    for idx in 0..input.len() {
        if idx >= window {
            //Unset old bit
            buffer ^= mask(input[idx - window]);
        }

        //Set new bit
        buffer ^= mask(input[idx]);

        //Currently window bits active
        if buffer.count_ones() >= window as u32 {
            return Some(idx + 1)
        }
    }

    None
}

pub fn _index_classical(input: &str, window: usize) -> Option<usize> {
    let mut last: Vec<Option<char>> = vec![None; window];

    for (idx, char) in input.chars().enumerate() {
        last.rotate_right(1);
        last[0] = Some(char);

        match last[1..].iter().position(|x| x == &Some(char)) {
            Some(x) => last[x + 1] = None,
            None => {
                if !last.contains(&None) {
                    return Some(idx + 1);
                }
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> usize {
    index_simd::<4>(input, 4).unwrap()
}

pub fn part_two(input: &str) -> usize {
    index_simd::<16>(input, 14).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_one(&input), 7);
    }

    #[test]
    fn test_alternatives() {
        use crate::read_file;
        let input = read_file("examples", 6);
        assert_eq!(index_simd::<8>(&input, 4), Some(7));
        assert_eq!(_index_popcnt(&input, 4), Some(7))
    }

    #[test]
    fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", 6);
        assert_eq!(part_two(&input), 19);
    }
}
