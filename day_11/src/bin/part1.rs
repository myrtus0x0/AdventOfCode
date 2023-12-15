use std::cmp;

fn get_expanded(map: Vec<char>, n_lines: usize) -> (Vec<usize>, Vec<usize>) {
    // TODO: can probably condence loops
    let expanding_rows: Vec<usize> = (0..(map.len() / n_lines))
        .filter(|row| {
            let mut is_empty = true;
            for i in 0..n_lines {
                if map[i + (row * n_lines)] == '#' {
                    is_empty = false;
                    break;
                }
            }
            is_empty
        })
        .collect();

    dbg!(expanding_rows.clone());

    let expanding_cols: Vec<usize> = (0..n_lines)
        .filter(|col| {
            let mut is_empty = true;
            for row in 0..(map.len() / n_lines) {
                if map[col + (row * n_lines)] == '#' {
                    is_empty = false;
                    break;
                }
            }
            is_empty
        })
        .collect();

    return (expanding_rows, expanding_cols);
}

fn part1(src_info: &str) -> i64 {
    let n_lines = src_info.lines().nth(0).unwrap().trim().len();
    let map: Vec<char> = src_info.replace("\n", "").chars().collect();
    let mut paths_sum = 0;

    let (expanding_rows, expanding_cols) = get_expanded(map.clone(), n_lines);

    let galaxies: Vec<(i64, i64)> = map
        .iter()
        .enumerate()
        .map(|(i, c)| (i % n_lines, i / n_lines, *c))
        .filter(|(_x, _y, c)| *c == '#')
        .map(|(x, y, _c)| (x as i64, y as i64))
        .collect();

    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let h_dist = (galaxies[j].0 - galaxies[i].0).abs();

            let horizontal_expansion_impacted = expanding_cols
                .iter()
                .filter(|n| {
                    let min = cmp::min(galaxies[i].0, galaxies[j].0);
                    let max = cmp::max(galaxies[i].0, galaxies[j].0);

                    (min..max).contains(&(**n as i64))
                })
                .count() as i64;

            let vertical_distance = (galaxies[j].1 - galaxies[i].1).abs();

            let vertical_expansion_impacted = expanding_rows
                .iter()
                .filter(|n| {
                    let min = cmp::min(galaxies[i].1, galaxies[j].1);
                    let max = cmp::max(galaxies[i].1, galaxies[j].1);

                    (min..max).contains(&(**n as i64))
                })
                .count() as i64;

            paths_sum += h_dist
                + horizontal_expansion_impacted
                + vertical_distance
                + vertical_expansion_impacted;
        }
    }

    return paths_sum;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part1(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374);
    }
}
