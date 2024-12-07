#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn rotate(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    pub fn step(self, pos: usize, line_len: usize, grid_len: usize) -> Option<usize> {
        match self {
            Dir::N => (pos >= line_len).then(|| pos - line_len),
            Dir::E => ((pos + 1) % line_len != 0).then(|| pos + 1),
            Dir::S => (pos + line_len < grid_len).then(|| pos + line_len),
            Dir::W => (pos % line_len != 0).then(|| pos - 1),
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let grid_size = rows * cols;
    let mut visited = vec![0u64; (grid_size + 63) / 64]; // Flat bitmask
    let mut visited_count = 0;

    let start_pos = input.bytes().position(|b| b == b'^').unwrap();
    let mut pos = start_pos;
    let mut dir = Dir::N;

    loop {
        if mark_visited(&mut visited, pos) {
            visited_count += 1;
        }

        if let Some(next_pos) = dir.step(pos, cols + 1, rows * (cols + 1)) {
            if input.as_bytes()[next_pos] == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            break;
        }
    }

    visited_count
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let grid_size = rows * cols;
    let start_pos = input.bytes().position(|b| b == b'^').unwrap();
    let mut obstacle_count = 0;

    for pos in 0..grid_size {
        if input.as_bytes()[pos] == b'#' || pos == start_pos || input.as_bytes()[pos] == b'\n' {
            continue;
        }

        if causes_loop(input.as_bytes(), cols + 1, rows * (cols + 1), start_pos, pos) {
            obstacle_count += 1;
        }
    }

    obstacle_count
}

fn mark_visited(visited: &mut [u64], pos: usize) -> bool {
    let idx = pos / 64;
    let bit = 1 << (pos % 64);
    if visited[idx] & bit == 0 {
        visited[idx] |= bit;
        true
    } else {
        false
    }
}

fn causes_loop(
    bytes: &[u8],
    line_len: usize,
    grid_len: usize,
    start_pos: usize,
    obstacle_pos: usize,
) -> bool {
    let mut visited_states = std::collections::HashSet::new();
    let mut pos = start_pos;
    let mut dir = Dir::N;

    loop {
        let state = (pos, dir);
        if !visited_states.insert(state) {
            return true; // Loop detected
        }

        if let Some(next_pos) = dir.step(pos, line_len, grid_len) {
            if next_pos == obstacle_pos || bytes[next_pos] == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            return false; // Exit the map
        }
    }
}
