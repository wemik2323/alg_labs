use image::*;

use l6::*;

const WIDTH: u32 = 800;
const CELL_WIDTH: u32 = 50;
const NUM_CELLS: u32 = WIDTH / CELL_WIDTH;

#[test]
fn iter_backtr_maze_gen_works() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, WIDTH);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 255, b]);
    }

    let maze = iter_backtr_maze_gen(NUM_CELLS as _, 0, 0);
    for x in 0..NUM_CELLS {
        for y in 0..NUM_CELLS {
            draw_cell(&mut imgbuf, maze[x as usize][y as usize], x, y);
        }
    }

    let path = std::env::args().nth(0).unwrap();
    let path = path.split_inclusive("target").next().unwrap().to_owned()
        + "/iter_backtr.png";

    imgbuf.save(&path).unwrap();
}

#[test]
fn aldous_broder_maze_gen_works() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, WIDTH);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * y as f32) as u8;
        let g = (0.3 * x as f32) as u8;
        *pixel = image::Rgb([r, g, 255]);
    }

    let maze = aldous_broder_maze_gen(NUM_CELLS as _, 0, 0);
    for x in 0..NUM_CELLS {
        for y in 0..NUM_CELLS {
            draw_cell(&mut imgbuf, maze[x as usize][y as usize], x, y);
        }
    }

    let path = std::env::args().nth(0).unwrap();
    let path = path.split_inclusive("target").next().unwrap().to_owned()
        + "/aldous_broder.png";

    imgbuf.save(&path).unwrap();
}

#[test]
fn rec_backtr_maze_gen_works() {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, WIDTH);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 255, b]);
    }

    let maze = rec_backtr_maze_gen(NUM_CELLS as _, 0, 0);
    for x in 0..NUM_CELLS {
        for y in 0..NUM_CELLS {
            draw_cell(&mut imgbuf, maze[x as usize][y as usize], x, y);
        }
    }

    let path = std::env::args().nth(0).unwrap();
    let path = path.split_inclusive("target").next().unwrap().to_owned()
        + "/rec_backtr.png";

    imgbuf.save(&path).unwrap();
}

fn draw_cell(
    buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    cell: u8,
    x: u32,
    y: u32,
) {
    let color = Rgb([80, 80, 80]);
    if cell & T == 0 {
        for j in 0..5 {
            for i in 0..CELL_WIDTH {
                buf[(i + y * CELL_WIDTH, j + x * CELL_WIDTH)] = color;
            }
        }
    }

    if cell & R == 0 {
        for j in 0..CELL_WIDTH {
            for i in 45..CELL_WIDTH {
                buf[(i + y * CELL_WIDTH, j + x * CELL_WIDTH)] = color;
            }
        }
    }

    if cell & B == 0 {
        for j in 45..CELL_WIDTH {
            for i in 0..CELL_WIDTH {
                buf[(i + y * CELL_WIDTH, j + x * CELL_WIDTH)] = color;
            }
        }
    }

    if cell & L == 0 {
        for j in 0..CELL_WIDTH {
            for i in 0..5 {
                buf[(i + y * CELL_WIDTH, j + x * CELL_WIDTH)] = color;
            }
        }
    }
}
