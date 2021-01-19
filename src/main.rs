mod grid;

use macroquad::prelude::*;
use crate::grid::{Grid, Cell};
use std::process::exit;

const CELL_SIZE:f32 = 10.;
const LINE_THICKNESS:f32 = 0.3;
const WIDTH:usize = 100;
const HEIGHT:usize = 10;

#[macroquad::main("gameoflife")]
async fn main() {

    let width = (screen_width() / CELL_SIZE) as usize;
    let height = (screen_height() / CELL_SIZE) as usize;

    let mut grid = Grid::new(width, height);

    /*
    grid.awaken(1,0);
    grid.awaken(2,1);
    grid.awaken(0,2);
    grid.awaken(1,2);
    grid.awaken(2,2);
    */

    grid.randomize(10000);

    loop {
        clear_background(BLACK);

        for (i, cell) in grid.cells.iter().enumerate() {

            match cell {
                Cell::Alive => {}
                Cell::Dead(false) => {
                    continue;
                }
                _ => {}
            }


            let color = {
                match cell {
                    Cell::Alive => {
                        WHITE
                    }
                    Cell::Dead(true) => {
                        RED
                    }
                    Cell::Dead(false) => {
                        BLACK
                    }
                }
            };


            draw_rectangle(
                CELL_SIZE * (i % width) as f32,
                CELL_SIZE * (i / width) as f32,
                CELL_SIZE,
                CELL_SIZE,
                color
            );

        }

        grid.step();

        next_frame().await;
    }
}
