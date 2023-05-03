pub const T: u8 = 1 << 0;
pub const R: u8 = 1 << 1;
pub const B: u8 = 1 << 2;
pub const L: u8 = 1 << 3;
const VISITED: u8 = 1 << 4;

#[inline]
pub fn iter_backtr_maze_gen(width: usize, x: usize, y: usize) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; width]; width];
    out[x][y] |= VISITED;
    let mut stack = vec![Cell::new(out[x][y], x, y)];

    while !stack.is_empty() {
        let mut curr_cell = stack.pop().unwrap();

        if let Some(mut r_n) = rand_neighbour(&out, &curr_cell, true) {
            remove_wall(&mut curr_cell, &mut r_n);
            stack.push(curr_cell);
            r_n.state |= VISITED;

            out[curr_cell.x_pos][curr_cell.y_pos] = curr_cell.state;
            out[r_n.x_pos][r_n.y_pos] = r_n.state;

            stack.push(r_n);
        }
    }

    out
}

#[inline]
pub fn aldous_broder_maze_gen(
    width: usize,
    x: usize,
    y: usize,
) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; width]; width];
    out[x][y] |= VISITED;
    let mut curr_cell = Cell::new(out[x][y], x, y);

    let mut unvis_cells = width * width - 1;

    while unvis_cells > 0 {
        let mut r_n = rand_neighbour(&out, &curr_cell, false).unwrap();

        if r_n.state == 0 {
            remove_wall(&mut curr_cell, &mut r_n);
            r_n.state |= VISITED;

            out[curr_cell.x_pos][curr_cell.y_pos] = curr_cell.state;
            out[r_n.x_pos][r_n.y_pos] = r_n.state;

            unvis_cells -= 1;
        }

        curr_cell = r_n;
    }

    out
}

#[inline]
pub fn rec_backtr_maze_gen(width: usize, x: usize, y: usize) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; width]; width];
    rec_backtr_maze_gen_impl(&mut out, x, y);

    out
}

#[inline]
fn rec_backtr_maze_gen_impl(maze: &mut [Vec<u8>], x: usize, y: usize) {
    maze[x][y] |= VISITED;
    let mut curr_cell = Cell::new(maze[x][y], x, y);
    while let Some(mut n) = rand_neighbour(maze, &curr_cell, true) {
        remove_wall(&mut curr_cell, &mut n);
        maze[x][y] = curr_cell.state;
        maze[n.x_pos][n.y_pos] = n.state;
        rec_backtr_maze_gen_impl(maze, n.x_pos, n.y_pos);
    }
}

use rand::prelude::*;
fn rand_neighbour(
    mat: &[Vec<u8>],
    cell: &Cell,
    unvis_only: bool,
) -> Option<Cell> {
    let mut out = Vec::new();

    let x = cell.x_pos;
    let y = cell.y_pos;

    // Верх
    if x > 0 {
        out.push(Cell::new(mat[x - 1][y], x - 1, y));
    }
    // Право
    if y < mat.len() - 1 {
        out.push(Cell::new(mat[x][y + 1], x, y + 1));
    }
    // Низ
    if x < mat.len() - 1 {
        out.push(Cell::new(mat[x + 1][y], x + 1, y));
    }
    // Лево
    if y > 0 {
        out.push(Cell::new(mat[x][y - 1], x, y - 1));
    }

    if unvis_only {
        out.retain(|c| c.state == 0);
    }

    if out.is_empty() {
        return None;
    }

    Some(out[thread_rng().gen_range(0..out.len())])
}

fn remove_wall(from: &mut Cell, to: &mut Cell) {
    if from.x_pos > to.x_pos {
        from.state |= T;
        to.state |= B;
    }
    if from.y_pos < to.y_pos {
        from.state |= R;
        to.state |= L;
    }
    if from.x_pos < to.x_pos {
        from.state |= B;
        to.state |= T;
    }
    if from.y_pos > to.y_pos {
        from.state |= L;
        to.state |= R;
    }
}

#[derive(Clone, Copy)]
struct Cell {
    state: u8,
    x_pos: usize,
    y_pos: usize,
}

impl Cell {
    fn new(state: u8, x_pos: usize, y_pos: usize) -> Self {
        Self {
            state,
            x_pos,
            y_pos,
        }
    }
}
