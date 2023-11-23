const TEST_HEIGHT: usize = 100;
const TEST_WIDTH: usize = 250;
const RA: isize = 21;

#[derive(Debug)]
struct Board {
    height: usize,
    width: usize,
    cells: Vec<Vec<f32>>,
}

impl Board {
    fn new(height: usize, width: usize) -> Self {
        let cells = vec![vec![0.0; width]; height];

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
        let gradient  = " ._>oa@#";
        for col in &self.cells {
                    println!("");
                    for row in col {
                        print!("{}", gradient.chars().nth((row*7.0).trunc() as usize).unwrap());
                    }
                }
    }

   fn next_iteration(&mut self) {
        unimplemented!();
   }

   fn compute_diff(&self) {
       for row  in &self.cells {
           for col in row {
               let mut m: f32 = 0.0;
               let mut n: f32 = 0.0;
               let mut M: f32 = 0.0;
               let mut N: f32 = 0.0;
               let RI = RA/3;

               for dy in -(RA - 1)..=(RA - 1) {
                   for dx in -(RA - 1)..=(RA - 1) {
                        println!("{} - {}", dy, dx);
                   }
               }


           }
       } 
   }

}

fn main() {
    let mut board = Board::new(TEST_HEIGHT, TEST_WIDTH);
    board.randomize();
    //board.display_board();
    board.compute_diff();
}
