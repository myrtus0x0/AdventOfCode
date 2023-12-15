use contiguous_mem::ContiguousMemory;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Ground,
    Start,
}

impl Direction {
    fn next_dir(&self, pipe: char) -> Option<Direction> {
        match (*self, pipe) {
            (Direction::North, '|') => Some(Direction::North),
            (Direction::North, '7') => Some(Direction::West),
            (Direction::North, 'F') => Some(Direction::East),
            (Direction::South, '|') => Some(Direction::South),
            (Direction::South, 'L') => Some(Direction::East),
            (Direction::South, 'J') => Some(Direction::West),
            (Direction::East, '-') => Some(Direction::East),
            (Direction::East, '7') => Some(Direction::South),
            (Direction::East, 'J') => Some(Direction::North),
            (Direction::West, '-') => Some(Direction::West),
            (Direction::West, 'L') => Some(Direction::North),
            (Direction::West, 'F') => Some(Direction::South),
            _ => None,
        }
    }
}

struct Grid {
    grid_vec: Vec<Vec<(Direction, Direction)>>,
    char_grid: Vec<Vec<char>>,
    n_rows: usize,
    n_columns: usize,
}

type Coord = (usize, usize);

impl Grid {
    fn new(input_grd: &str) -> Grid {
        let lines = input_grd.lines().collect::<Vec<&str>>();
        let map_width = lines[0].len();
        let mut grid: Vec<Vec<(Direction, Direction)>> =
            vec![vec![(Direction::Ground, Direction::Ground); map_width]; lines.len()];
        lines.iter().enumerate().for_each(|(r, line)| {
            line.chars().enumerate().for_each(|(c, char)| {
                let dir = match char {
                    '|' => (Direction::North, Direction::South),
                    '-' => (Direction::East, Direction::West),
                    'L' => (Direction::North, Direction::East),
                    'J' => (Direction::North, Direction::West),
                    '7' => (Direction::South, Direction::West),
                    'F' => (Direction::South, Direction::East),
                    'S' => (Direction::Start, Direction::Start),
                    _ => (Direction::Ground, Direction::Ground),
                };
                grid[r][c] = dir;
            });
        });

        let mut char_grid: Vec<Vec<char>> = vec![vec!['.'; map_width]; lines.len()];
        lines.iter().enumerate().for_each(|(r, line)| {
            line.chars().enumerate().for_each(|(c, char)| {
                char_grid[r][c] = char;
            });
        });
        let new_grid = Grid {
            grid_vec: grid.clone(),
            char_grid: char_grid,
            n_rows: grid.len(),
            n_columns: grid[0].len(),
        };

        return new_grid;
    }

    fn find_start(&self) -> Option<Coord> {
        for (r, row) in self.grid_vec.iter().enumerate() {
            for (c, point) in row.iter().enumerate() {
                if point.0 == Direction::Start && point.1 == Direction::Start {
                    return Some((r, c));
                }
            }
        }

        return None;
    }

    fn get_next(&self, current_loc: Coord, dir_to_follow: Direction) -> Option<Coord> {
        let (r, c) = current_loc;
        match dir_to_follow {
            Direction::North => r.checked_add_signed(-1).map(|r| (r, c)),
            Direction::South => r.checked_add_signed(1).map_or(None, |r| {
                if r >= self.n_rows {
                    None
                } else {
                    Some((r, c))
                }
            }),
            Direction::East => c.checked_add_signed(1).map_or(None, |c| {
                if c >= self.n_columns {
                    None
                } else {
                    Some((r, c))
                }
            }),
            Direction::West => c.checked_add_signed(-1).map(|c| (r, c)),
            Direction::Ground => None,
            Direction::Start => None,
        }
    }

    fn path_length(
        &self,
        start: Coord,
        mut dir: Direction,
    ) -> (usize, bool, Vec<(Coord, Direction)>) {
        let mut path_length = 1;
        let mut does_loop = false;
        let mut full_path = Vec::new();

        let mut current_loc = (0, 0);
        full_path.push((start, dir));

        match self.get_next(start, dir) {
            Some(temp) => current_loc = temp,
            None => return (path_length, does_loop, full_path),
        };

        // check if the direction we went is in the next location we reached,
        // if not then we return as its not a valid path
        if self.grid_vec[current_loc.0][current_loc.1].0 != dir
            && self.grid_vec[current_loc.0][current_loc.1].1 != dir
        {
            return (path_length, does_loop, full_path);
        }

        loop {
            let current_rune = self.char_grid[current_loc.0][current_loc.1];

            // if we are back at start we've looped
            if current_loc.0 == start.0 && current_loc.1 == start.1 {
                does_loop = true;
                break;
            }

            dir = dir.next_dir(current_rune).unwrap();
            match self.get_next(current_loc, dir) {
                Some(temp) => current_loc = temp,
                None => return (path_length, does_loop, full_path),
            };

            full_path.push((current_loc, dir));
            path_length += 1;
        }

        return (path_length, does_loop, full_path);
    }
}

pub fn part2(src_info: &str) -> usize {
    let grid = Grid::new(src_info);
    let start = grid.find_start().unwrap();
    let all_paths = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let (direction, length, path) = all_paths
        .into_iter()
        .filter_map(|dir| {
            let (length, loops, path) = grid.path_length(start, dir);
            if loops {
                Some((dir, length, path))
            } else {
                None
            }
        })
        .max_by_key(|(_, l, _)| *l)
        .unwrap();

    let mut coordinates = Vec::new();
    for (coordinate, _) in path {
        coordinates.push(coordinate);
    }

    return 0;
}

fn main() {
    let input = include_str!("../puzzle");
    let answer = part2(input);
    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct() {
        let result = part2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 4);
    }
}
