use std::f32::consts::E;
use std::thread::sleep;
use std::time::{Duration, Instant};

const TEST_HEIGHT: i32 = 100;
const TEST_WIDTH: i32 = 250;
const RA: isize = 21;
const ALPHA_N: f32 = 0.028;
const ALPHA_M: f32 = 0.147;
const B1: f32 = 0.278;
const B2: f32 = 0.365;
const D1: f32 = 0.267;
const D2: f32 = 0.445;
const DT: f32 = 0.1;

fn gmod(x: i32, y: i32) -> usize {
    (((x % y) + y) % y) as usize
}

fn sigma1(x: f32, a: f32, alpha: f32) -> f32 {
    1.0 / (1.0 + E.powf(-(x - a) * 4.0 / alpha))
}

fn sigma2(x: f32, a: f32, b: f32) -> f32 {
    sigma1(x, a, ALPHA_N) * (1.0 - sigma1(x, b, ALPHA_N))
}

fn sigmam(x: f32, y: f32, m: f32) -> f32 {
    x * (1.0 - sigma1(m, 0.5, ALPHA_M)) + y * sigma1(m, 0.5, ALPHA_M)
}

fn s(n: f32, m: f32) -> f32 {
    sigma2(n, sigmam(B1, D1, m), sigmam(B2, D2, m))
}

fn clamp(x: f32, l: f32, h: f32) -> f32 {
    if x < l {
        l
    } else if x > h {
        h
    } else {
        x
    }
}

#[derive(Debug)]
struct Board {
    height: i32,
    width: i32,
    cells: Vec<Vec<f32>>,
}

impl Board {
    fn new(height: i32, width: i32) -> Self {
        let cells = vec![vec![0.0; width as usize]; height as usize];

        Self {
            height,
            width,
            cells,
        }
    }

    fn randomize(&mut self) {
        for row in self.cells.iter_mut() {
            for cols in row.iter_mut() {
                *cols = rand::random::<f32>();
            }
        }
    }

    fn display_board(&self) {
        let gradient = " ._>oa@#";
        for col in &self.cells {
            println!("");
            for row in col {
                print!(
                    "{}{}",
                    gradient.chars().nth((row * 7.0).trunc() as usize).unwrap(),
                    gradient.chars().nth((row * 7.0).trunc() as usize).unwrap()
                );
            }
        }
    }

    #[allow(non_snake_case)]
    fn compute_diff(&mut self) {
        let mut grid = self.cells.clone();
        for col in 0..self.height {
            for row in 0..self.width {
                let mut m: f32 = 0.0;
                let mut n: f32 = 0.0;
                let mut M = 0;
                let mut N = 0;
                let RI = RA / 3;

                for dy in -(RA - 1)..=(RA - 1) {
                    for dx in -(RA - 1)..=(RA - 1) {
                        let x = gmod(col + dx as i32, self.width);
                        let y = gmod(row + dy as i32, self.height);

                        if dx * dx + dy * dy <= RI * RI {
                            m += &self.cells[y][x];
                            M += 1;
                        } else if dx * dx + dy * dy <= RA * RA {
                            n += &self.cells[y][x];
                            N += 1;
                        }
                    }
                }
                n /= N as f32;
                m /= M as f32;

                let q = s(n, m);
                grid[col as usize][row as usize] = 2.0 * q - 1.0;
                self.cells[col as usize][row as usize] += DT * grid[col as usize][row as usize];
                self.cells[col as usize][row as usize] =
                    clamp(self.cells[col as usize][row as usize], 0.0, 1.0)
            }
        }
    }
}

fn main() {
    let mut board = Board::new(TEST_HEIGHT, TEST_WIDTH);
    board.randomize();

    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;

    loop {
        clearscreen::clear().expect("failed to clear screen");
        board.display_board();
        board.compute_diff();
    }
}
